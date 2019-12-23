use serde::{Deserialize, Serialize};
use bitflags::bitflags;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Protocols: u32 {
        const HTTP =  0b00000001;
        const HTTPS = 0b00000010;
        const ALL = 0b00000011;
    }
}

impl std::str::FromStr for Protocols {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Protocols::empty();
        let split: Vec<&str> = s.split(',').collect();
        if split.contains(&"https") {
            result = result | Protocols::HTTPS;
        }
        if split.contains(&"http") {
            result = result | Protocols::HTTP;
        }
        Ok(result)
    }
}

impl ToString for Protocols {
    fn to_string(&self) -> String {
        let result = match *self {
            Protocols::HTTP => "http",
            Protocols::HTTPS => "http",
            Protocols::ALL => "http,https",
            _ => panic!("Unknown protocol")
        };
        result.to_string()
    }
}

impl ToString for Mirror {
    fn to_string(&self) -> String {
        format!("Server = {}$repo/os/$arch", self.url.as_ref().unwrap())
    }
}

//TODO: Make these types more rusty, (ie. uri -> url::Url type)
#[derive(Clone, Deserialize, Debug)]
pub struct Mirror {
    pub url: Option<String>,
    pub protocol: Option<String>,
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

#[derive(Clone, Deserialize, Debug)]
pub struct Mirrors {
    cutoff: u64,
    last_check: String,
    num_checks: Option<u64>,
    check_frequency: Option<u64>,
    urls: Vec<Mirror>,
    version: u64,
}

impl IntoIterator for Mirrors {
    type Item = Mirror;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.urls.into_iter()
    }
}
