use crate::Protocols;
use anyhow::Result;
use byte_unit::Byte;
use chrono::prelude::*;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use tokio::prelude::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Mirror {
    pub url: String,
    pub protocol: Protocols,
    pub last_sync: Option<DateTime<Utc>>,
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
    pub rate: Option<RateResult>,
}

impl Display for Mirror {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rate: String = if let Some(rate) = &self.rate {
            rate.to_pretty()
        } else {
            "".into()
        };
        write!(
            f,
            "Server = {}$repo/os/$arch\n#        ↳ {}",
            self.url, rate
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RateResult {
    num_bytes: f64,
    num_millis: f64,
}

impl RateResult {
    pub fn to_bytes_per_sec(&self) -> f64 {
        self.num_bytes / self.num_millis
    }
}

impl PartialOrd for RateResult {
    fn partial_cmp(&self, other: &RateResult) -> std::option::Option<std::cmp::Ordering> {
        other.to_bytes_per_sec()
            .partial_cmp(&self.to_bytes_per_sec())
    }
}

impl RateResult {
    pub fn new(num_bytes: f64, num_millis: f64) -> Self {
        RateResult {
            num_bytes,
            num_millis,
        }
    }

    pub fn to_pretty(&self) -> String {
        let byte_unit: Byte = Byte::from_bytes(self.to_bytes_per_sec() as u128);
        format!("{}/s", byte_unit.get_appropriate_unit(true))
    }
}

impl Display for RateResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_pretty())
    }
}

impl Mirror {
    pub async fn rate<'a>(&'a mut self) -> Result<()> {
        let url = format!("{}{}", self.url, "core/os/x86_64/core.db");

        let now = std::time::Instant::now();
        if let Ok(resp) = reqwest::get(&*url).await {
            let num_bytes = resp.bytes().await?.len() as f64;
            let num_millis = now.elapsed().as_secs_f64();

            self.rate = Some(RateResult::new(num_bytes, num_millis));
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
