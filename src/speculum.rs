mod mirror;
mod mirrors;
mod protocols;

use anyhow::anyhow;
use dirs::cache_dir;
use log::info;
use reqwest::Client;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::from_utf8;
use std::time::{Duration, SystemTime};
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

    fn get_cache_path(&self) -> PathBuf {
        let mut path = cache_dir().unwrap_or_default();
        path.push("mirrorstatus.json");
        path
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors> {
        let cache_path = self.get_cache_path();

        let metadata = fs::metadata(&cache_path).await;
        let invalid = match metadata {
            Ok(meta) => {
                let duration_since = SystemTime::now().duration_since(meta.modified()?)?;
                info!("{:?}", duration_since);
                duration_since > Duration::from_secs(300)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => true,
            Err(_) => true,
        };

        let mut mirrors: Mirrors = if invalid {
            info!("Using new");
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&cache_path)
                .await?;
            let request = self
                .client
                .get(URL)
                .send()
                .await?
                .text_with_charset("UTF-8")
                .await?;
            file.write_all(request.as_bytes()).await?;
            serde_json::from_str(&request)?
        } else {
            info!("Using cache");
            let mut file = fs::File::open(cache_path).await?;
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf).await?;
            serde_json::from_str(from_utf8(&buf)?)?
        };

        mirrors.get_urls_mut().retain(|url| url.score.is_some());

        Ok(mirrors)
    }
}
