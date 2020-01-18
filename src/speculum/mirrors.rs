use crate::{Mirror, Protocols};
use anyhow::Result;

use serde::Deserialize;
use std::path::Path;
use tokio::fs::{File, OpenOptions};
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
    pub async fn load_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Mirrors> {
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::open(path).await?;

        file.read_to_end(&mut buf).await?;

        Mirrors::load_from_utf8(buf)
    }

    pub fn load_from_utf8<P: AsRef<[u8]>>(buf: P) -> Result<Mirrors>
    {
        let mut mirrors: Mirrors = serde_json::from_slice(buf.as_ref())?;
        mirrors.get_urls_mut().retain(|url| url.score.is_some());
        Ok(mirrors)
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

    pub fn get_urls(&'a mut self) -> &'a Vec<Mirror> {
        &self.urls
    }

    pub fn filter_protocols(&'a mut self, p: Protocols) -> &'a mut Self {
        self.urls.retain(|url| url.protocol.intercects(p));
        self
    }

    pub fn take(&'a mut self, n: usize) -> &'a mut Self {
        self.get_urls_mut().truncate(n);
        self
    }

    /// Saves the recieved mirrorlist in pacman format
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

    pub async fn write<W: AsyncWrite + Unpin>(&mut self, fd: &mut W) -> Result<()> {
        let urls = &self.get_urls();
        for url in urls.into_iter() {
            fd.write(format!("{}\n", &url.to_string()).as_bytes())
                .await?;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.urls.len()
    }
}
