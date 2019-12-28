use crate::Mirror;
use crate::Protocols;

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::prelude::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Mirrors {
    cutoff: u64,
    last_check: String,
    num_checks: u64,
    check_frequency: u64,
    urls: Vec<Mirror>,
    version: u64,
}

impl<'a> Mirrors {
    pub fn order_by<F>(&'a mut self, order: F) -> &'a mut Self
    where
        F: FnMut(&Mirror, &Mirror) -> std::cmp::Ordering,
    {
        self.urls.sort_by(order);
        self
    }

    pub fn filter_protocols(&'a mut self, p: Protocols) -> &'a mut Self {
        self.urls.retain(|url| url.protocol.intercects(p));
        self
    }

    pub fn take(&'a mut self, n: Option<usize>) -> &'a mut Self {
        if let Some(n) = n {
            let urls = &mut self.urls;
            urls.truncate(n);
        }
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
}
