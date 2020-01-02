mod mirror;
mod mirrors;
mod protocols;

use reqwest::Client;

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;

pub use mirror::Mirror;
pub use mirrors::Mirrors;
pub use protocols::Protocols;

use tokio::fs::{OpenOptions, metadata};
use tokio::io::AsyncWriteExt;
use std::path::Path;

use dirs;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

pub struct Speculum<'a> {
    client: reqwest::Client,
    mirrors: Option<&'a Mirrors>,
}

impl Default for Speculum<'_> {
    fn default() -> Self {
        Speculum {
            client: Client::new(),
            mirrors: None
        }
    }
}

impl<'a> Speculum<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn fetch_mirrors(&'a mut self) -> Result<&'a mut Self> {
        let mirrors: Mirrors = self.client.get(URL).send().await?.json::<Mirrors>().await?;
        if let Some(mut cache_dir) = dirs::cache_dir()
        {
            cache_dir.push("mirrorstatus.json");
            let mtime = metadata(cache_dir).await?.modified().unwrap();
            let diff = std::time::SystemTime::now().duration_since(mtime).unwrap().as_secs();
            if diff > 300
            {
                self
                    .get_urls_mut()
                    .retain(|url| url.score.is_some());
            }
        }

        self.mirrors = Some(&mirrors);
        Ok(self)
    }

    pub fn order_by<F>(&'a mut self, order: F) -> &'a mut Self
    where
        F: FnMut(&Mirror, &Mirror) -> std::cmp::Ordering,
    {
        self.mirrors.unwrap().urls.sort_unstable_by(order);
        self
    }

    pub fn get_urls_mut(&'a mut self) -> &'a mut Vec<Mirror> {
        self.mirrors.unwrap().urls.as_mut()
    }

    pub fn filter_protocols(&'a mut self, p: Protocols) -> &'a mut Self {
        self.mirrors.unwrap().urls.retain(|url| url.protocol.intercects(p));
        self
    }

    pub fn take(&'a mut self, n: usize) -> &'a mut Self {
        self.get_urls_mut().truncate(n);
        self
    }

    pub async fn save<P: AsRef<Path>>(&'a mut self, path: P) -> Result<()> {
        if let Some(mirrors) = &self.mirrors
        {
            let mirrors = mirrors.urls;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .await?;

        let fetched = self.get_urls_mut()
            .into_iter()
            .map(|url| url.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        file.write(fetched.as_bytes()).await?;
        file.sync_data().await?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.mirrors.unwrap().urls.len()
    }
}
