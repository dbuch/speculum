use crate::Protocols;
use anyhow::Result;
use serde::Deserialize;
use tokio::prelude::*;
use std::fmt::{self, Display, Formatter};

impl Display for Mirror
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "Server = {}$repo/os/$arch", self.url)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Mirror {
    pub url: String,
    pub protocol: Protocols,
    pub last_sync: Option<String>,
    pub completion_pct: f64,
    pub delay: Option<u64>,
    pub duration_avg: Option<f64>,
    pub duration_stddev: Option<f64>,
    pub score: Option<f64>,
    pub active: Option<bool>,
    pub country: String,
    pub country_code: String,
    pub isos: Option<bool>,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: Option<String>,
}

impl Mirror {
    pub async fn rate(&self) -> Result<std::time::Duration> {
        //TODO: should rate, by downloading core.db
        unimplemented!();
    }
    
    pub async fn write<T>(&self, target: &mut T) -> Result<()>
    where T: AsyncWrite + Unpin
    {
        target.write(self.url.as_bytes()).await?;
        Ok(())
    }
}
