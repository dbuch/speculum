use serde::Deserialize;

#[serde(default, from = "String")]
#[derive(Copy, Deserialize, Clone, Debug)]
pub struct Protocols {
    pub http: bool,
    pub https: bool,
    pub rsync: bool,
}

impl Default for Protocols {
    fn default() -> Protocols {
        Protocols {
            http: true,
            https: true,
            rsync: true,
        }
    }
}

impl Protocols {
    pub fn intercects(&self, other: Protocols) -> bool
    {
        self.http & other.http || self.https & other.https || self.rsync & other.rsync
    }
}

impl From<&str> for Protocols {
    fn from(s: &str) -> Self
    {
        let split = s.split(',').collect::<Vec<&str>>();
        Protocols {
            http: split.contains(&"http"),
            https: split.contains(&"https"),
            rsync: split.contains(&"rsync"),
        }
    }
}

impl From<String> for Protocols {
    fn from(s: String) -> Self {
        let split = s.split(',').collect::<Vec<&str>>();
        Protocols {
            http: split.contains(&"http"),
            https: split.contains(&"https"),
            rsync: split.contains(&"rsync"),
        }
    }
}

impl std::str::FromStr for Protocols {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(',').collect::<Vec<&str>>();
        Ok(Protocols {
            http: split.contains(&"http"),
            https: split.contains(&"https"),
            rsync: split.contains(&"rsync"),
        })
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

    pub fn filter_protocols(&'a mut self, p: Protocols) -> &'a mut Self
    {
        self.urls.retain(|url| url.protocol.intercects(p));
        self
    }

    pub fn protocols<F>(&'a mut self, protocols: F) -> &'a mut Self
    where
        F: Fn(&Protocols) -> bool,
    {
        let urls = &mut self.urls;
        for i in 0..urls.len() {
            if protocols(&mut urls[i].protocol) {
                urls.remove(i);
            }
        }
        self
    }
}

impl IntoIterator for Mirrors {
    type Item = Mirror;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.urls.into_iter()
    }
}
