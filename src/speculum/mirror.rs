use crate::{Protocols, Result};
use serde::Deserialize;

//TODO: We ought to have something smarter, like serialize implatation of mirrorlist
//      It's a very simple format.
impl ToString for Mirror {
    fn to_string(&self) -> String {
        format!("Server = {}$repo/os/$arch", &self.url.clone().unwrap())
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Mirror {
    pub url: Option<String>,
    pub protocol: Protocols,
    pub last_sync: Option<String>,
    pub completion_pct: Option<f64>,
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
    pub details: Option<String>,
}

impl Mirror {
    pub async fn rate(&self) -> Result<std::time::Duration> {
        //TODO: should rate, by downloading core.db
        unimplemented!();
    }
}
