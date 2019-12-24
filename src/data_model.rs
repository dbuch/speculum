use serde::Deserialize;

#[serde(rename_all = "lowercase")]
#[derive(Copy, PartialEq, Deserialize, Clone, Debug)]
pub enum Protocol {
    Http,
    Https,
    Rsync,
}

impl std::str::FromStr for Protocol {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prot = match s {
            "http" => Protocol::Http,
            "https" => Protocol::Https,
            "rsync" => Protocol::Rsync,
            _ => panic!("unknown protocol")
        };
        Ok(prot)
    }
}

impl ToString for Mirror {
    fn to_string(&self) -> String {
        format!("Server = {}$repo/os/$arch", &self.url.clone().unwrap())
    }
}

//TODO: Make these types more rusty, (ie. uri -> url::Url type)
#[derive(Clone, Deserialize, Debug)]
pub struct Mirror {
    pub url: Option<String>,
    pub protocol: Option<Protocol>,
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
