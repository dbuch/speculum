mod mirror;
mod mirrors;
mod protocols;

use anyhow::anyhow;
use dirs::cache_dir;
use log::info;
use reqwest::Client;
use std::time::SystemTime;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;

pub use mirror::Mirror;
pub use mirrors::Mirrors;
pub use protocols::Protocols;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

pub struct Speculum {
    client: reqwest::Client,
}

impl Default for Speculum {
    fn default() -> Self {
        Speculum {
            client: Client::new(),
        }
    }
}

impl Speculum {
    pub fn new() -> Self {
        Self::default()
    }

    async fn get_cache_file(&self) -> Result<fs::File> {
        let mut cached = cache_dir().unwrap();
        cached.push("mirrorstatus.json");
        let result = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(cached)
            .await;
        result.map_err(|e| anyhow!(e))
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors> {
        let mut cache_file = self.get_cache_file().await?;

        let meta = &cache_file.metadata().await;
        if let Ok(m) = meta {
            let cache_age = SystemTime::now().duration_since(m.modified()?)?;

            info!("Cache {:?}", cache_age);
            if cache_age.as_secs() > 300 {
                info!("Found valid cache");
                let mut content: Vec<u8> = Vec::new();

                cache_file.read_to_end(&mut content).await?;
                let mirrors: Mirrors = serde_json::from_str(std::str::from_utf8(&content)?)?;
                return Ok(mirrors);
            }
        }

        let mirrors_status = self.client.get(URL).send().await?;
        let mirrors_bytes = mirrors_status.text().await?;
        cache_file.write_all(mirrors_bytes.as_bytes()).await?;

        let mut mirrors: Mirrors = serde_json::from_str(&mirrors_bytes)?;
        mirrors
            .get_urls_mut()
            .retain(|url| url.score.is_some() && url.active.unwrap());
        Ok(mirrors)
    }
}
