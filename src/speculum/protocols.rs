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
