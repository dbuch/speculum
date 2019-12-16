use chrono;
use chrono::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol
{
    Https,
    Http,
    Rsync,
    Unknown,
}

impl Into<String> for Protocol
{
    fn into(self: Self) -> String
    {
        let res = match self {
            Protocol::Https => "https",
            Protocol::Http => "http",
            Protocol::Rsync => "rsync",
            _ => "unknown"
        };
        res.to_string()
    }
}

impl From<String> for Protocol
{
    fn from(s: String) -> Protocol
    {
        match s.as_ref() {
            "https" => Protocol::Https,
            "http" => Protocol::Http,
            "rsync" => Protocol::Rsync,
            _ => Protocol::Unknown,
        }
    }
}

impl Mirror {
    pub fn get_coredb_url(&self) -> String
    {
        let mut string = self.url.as_ref().unwrap().to_string();
        string.push_str("core/os/x86_64/core.db");
        string
    }
}


#[derive(Clone, Deserialize, Debug)]
pub struct Mirror {
    pub url: Option<String>,
    pub protocol: Protocol,
    pub last_sync: Option<DateTime<Utc>>,
    pub completion_pct: f64,
    pub delay: Option<u64>,
    pub duration_avg: Option<f64>,
    pub duration_stddev: Option<f64>,
    pub score: Option<f64>,
    pub active: Option<bool>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub isos: Option<bool>,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: Option<String>
}
