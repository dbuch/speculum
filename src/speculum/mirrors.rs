use crate::{Mirror, Protocols, Result};

use serde::Deserialize;
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::prelude::*;

/// Contains information about Mirrors.
///
/// Filter actions and take is avalible throug this interface
#[derive(Clone, Deserialize, Debug)]
pub struct Mirrors {
    cutoff: u64,
    last_check: String,
    num_checks: u64,
    check_frequency: u64,
    urls: Vec<Mirror>,
    version: u64,
}

impl Mirrors {
    pub async fn load_from<P: AsRef<std::path::Path>>(path: P) -> Result<Mirrors> {
        let mut file = tokio::fs::File::open(path).await?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).await?;
        Ok(serde_json::from_str(std::str::from_utf8(&buf)?)?)
    }
}

impl<'a> Mirrors {
    pub fn order_by<F>(&'a mut self, order: F) -> &'a mut Self
    where
        F: FnMut(&Mirror, &Mirror) -> std::cmp::Ordering,
    {
        self.urls.sort_unstable_by(order);
        self
    }

    pub fn get_urls_mut(&'a mut self) -> &'a mut Vec<Mirror> {
        self.urls.as_mut()
    }

    pub fn filter_protocols(&'a mut self, p: Protocols) -> &'a mut Self {
        self.urls.retain(|url| url.protocol.intercects(p));
        self
    }

    pub fn take(&'a mut self, n: usize) -> &'a mut Self {
        self.get_urls_mut().truncate(n);
        self
    }

    pub async fn save<P: AsRef<Path>>(&'a mut self, path: P) -> Result<()> {
        let urls = &self.urls;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .await?;

        let fetched = urls
            .into_iter()
            .map(|url| url.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        file.write(fetched.as_bytes()).await?;
        file.sync_data().await?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.urls.len()
    }
}
