use crate::{Mirror, Protocols};
use anyhow::Result;
use log::info;
use std::fmt::Debug;

use serde::Deserialize;
use tokio::fs::File;
use tokio::prelude::*;
use futures::future::try_join_all;

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

    pub fn load_from_utf8<P: AsRef<[u8]>>(buf: P) -> Result<Mirrors> {
        let mut mirrors: Mirrors = serde_json::from_slice(buf.as_ref())?;
        mirrors
            .get_urls_mut()
            .retain(|url| url.completion_pct.ne(&0.0f64) && !f64::is_nan(url.completion_pct));
        Ok(mirrors)
    }
}

impl<'a> Mirrors {
    pub fn order_by<F>(&'a mut self, order: F) -> &'a mut Self
    where
        F: FnMut(&Mirror, &Mirror) -> std::cmp::Ordering,
    {
        info!("Ording entries");
        self.urls.sort_unstable_by(order);
        self
    }

    pub fn get_urls_mut(&'a mut self) -> &'a mut Vec<Mirror> {
        self.urls.as_mut()
    }

    pub fn get_urls(&'a mut self) -> &'a Vec<Mirror> {
        &self.urls
    }

    pub fn filter_protocols(&'a mut self, p: impl Into<Protocols>) -> &'a mut Self {
        let protocol: Protocols = p.into();
        info!("Applying protocol filtering {:?}", protocol);
        self.urls.retain(|url| url.protocol.intercects(protocol));
        self
    }

    pub fn take(&'a mut self, n: usize) -> &'a mut Self {
        info!("Taking {} entries", n);
        self.get_urls_mut().truncate(n);
        self
    }

    /// Writes the recieved mirrorlist in pacman format a file
    pub async fn write<W: AsyncWrite + Unpin + Debug>(&mut self, fd: &mut W) -> Result<()> {
        info!("Writing to {:?}", *fd);
        for url in self.get_urls().into_iter() {
            fd.write(format!("{}\n", &url).as_bytes()).await?;
        }
        fd.shutdown().await?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.urls.len()
    }

    pub async fn rate_all(&'a mut self) -> Result<&'a mut Self>
    {
        let mut rate = Vec::new();
        for mirror in self.get_urls_mut()
        {
            rate.push(mirror.rate());
        }

        try_join_all(rate).await?;
        
        Ok(self)
    }
}
