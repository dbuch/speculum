mod mirror;
mod mirrors;
mod protocols;
mod utils;

use anyhow::{Result, bail};
use dirs::cache_dir;
use log::*;
use reqwest::Client;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::prelude::*;

pub use mirror::Mirror;
pub use mirrors::Mirrors;
pub use protocols::Protocols;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

/// Toplevel object to receieve mirror status
pub struct Speculum {
    client: reqwest::Client,
    cache_timeout: u64,
}

impl Default for Speculum {
    fn default() -> Self {
        Speculum {
            client: Client::new(),
            cache_timeout: 300,
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
            cache_timeout: Speculum::default().cache_timeout,
        })
    }

    pub fn get_cache_path(&self) -> Result<PathBuf> {
        if let Some(mut path) = cache_dir() {
            path.push("mirrorstatus.json");
            return Ok(path);
        }

        bail!("Unable to get user cache directory")
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors> {
        let cache_path = self.get_cache_path()?;

        let metadata = fs::metadata(&cache_path).await;
        let invalid = match metadata {
            Ok(meta) => {
                SystemTime::now().duration_since(meta.modified()?)?
                    > Duration::from_secs(self.cache_timeout)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => true,
            Err(e) => bail!(e),
        };

        let mut mirrors: Mirrors = if invalid {
            info!("Fetching status json..");
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
            info!("Using cached status json..");
            Mirrors::load_from(cache_path).await?
        };

        mirrors.get_urls_mut().retain(|url| url.score.is_some());

        Ok(mirrors)
    }
}
