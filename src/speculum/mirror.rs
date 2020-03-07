use crate::Protocols;
use anyhow::Result;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use tokio::prelude::*;

impl Display for Mirror {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Server = {}$repo/os/$arch {}", self.url, self.rate)
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
    #[serde(skip)]
    pub rate: RateResult,
}

#[derive(Clone, Debug)]
pub struct RateResult {
    num_bytes: u128,
    num_millis: u128,
}

impl Default for RateResult {
    fn default() -> Self {
        RateResult::new(std::u128::MAX, std::u128::MAX)
    }
}

impl RateResult {
    pub fn new(num_bytes: u128, num_millis: u128) -> Self {
        RateResult {
            num_bytes,
            num_millis,
        }
    }

    pub fn is_rated(&self) -> bool
    {
        self.num_millis != std::u128::MAX && self.num_millis != std::u128::MAX
    }
}

impl Display for RateResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_rated()
        {
            write!(f, "# {:.2} KiB/s", (self.num_bytes as f64 / (self.num_millis as f64 / 1000.0)) / 1000.0)
        }
        else
        {
            write!(f, "# No rate")
        }
    }
}

impl Mirror {
    pub async fn rate<'a>(&'a mut self) -> Result<()> {
        let url = format!("{}{}", self.url, "core/os/x86_64/core.db");

        let now = std::time::Instant::now();
        if let Ok(resp) = reqwest::get(&*url).await {
            let num_bytes = resp.bytes().await?.len() as u128;
            let num_millis = now.elapsed().as_millis();

            self.rate = RateResult::new(num_bytes, num_millis);
        }

        Ok(())
    }

    pub async fn write<T>(&self, target: &mut T) -> Result<()>
    where
        T: AsyncWrite + Unpin,
    {
        target.write(self.url.as_bytes()).await?;
        Ok(())
    }
}
