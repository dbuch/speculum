mod mirror;
mod mirrors;
mod protocols;

use dirs::cache_dir;
use log::info;
use reqwest::Client;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::{
    fs,
    io::AsyncWriteExt,
};

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;

pub use mirror::Mirror;
pub use mirrors::Mirrors;
pub use protocols::Protocols;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

pub struct Speculum {
    client: reqwest::Client,
    cache_age: u64,
}

impl Default for Speculum {
    fn default() -> Self {
        Speculum {
            client: Client::new(),
            cache_age: 300,
        }
    }
}

impl Speculum {
    pub fn new() -> Result<Self> {
        Ok(Self::default())
    }

    pub fn with(connection_timeout: u64) -> Result<Self> {
        Ok(Speculum {
            client: Client::builder()
                .connect_timeout(Duration::from_secs(connection_timeout))
                .build()?,
            cache_age: Speculum::default().cache_age,
        })
    }

    pub fn get_cache_path(&self) -> PathBuf {
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
                duration_since > Duration::from_secs(self.cache_age)
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
            Mirrors::load_from(cache_path).await?
        };

        mirrors.get_urls_mut().retain(|url| url.score.is_some());

        Ok(mirrors)
    }
}
