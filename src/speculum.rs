use chrono;
use bytes::buf::BufExt as _;
use serde::Deserialize;
use serde_json::from_reader;
use std::rc::Rc;
use hyper::{
    client::Client,
    client::HttpConnector,
    body::{
        Body,
        aggregate,
    },
};
use hyper_tls::HttpsConnector;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Https,
    Http,
    Rsync,
    Unknown,
}

impl Into<String> for Protocol {
    fn into(self: Self) -> String {
        let res = match self {
            Protocol::Https => "https",
            Protocol::Http => "http",
            Protocol::Rsync => "rsync",
            _ => panic!("Protocol not supported"),
        };
        res.to_string()
    }
}

impl From<String> for Protocol {
    fn from(s: String) -> Protocol {
        match s.as_ref() {
            "https" => Protocol::Https,
            "http" => Protocol::Http,
            "rsync" => Protocol::Rsync,
            _ => panic!("Protocol not supported"),
        }
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

impl IntoIterator for Mirrors
{
    type Item = Mirror;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter
    {
        self.urls.into_iter()
    }
}

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Speculum
{
    https_client: Rc<hyper::Client<HttpsConnector<HttpConnector>, Body>>,
    http_client: Rc<hyper::Client<HttpConnector, Body>>
}

impl Speculum {
    pub fn new() -> Self
    {
        let https = HttpsConnector::new();
        Speculum {
            https_client: 
                Rc::new(Client::builder()
                    .keep_alive_timeout(std::time::Duration::new(5, 0))
                    .build(https)),
            http_client: 
                Rc::new(Client::builder()
                    .keep_alive_timeout(std::time::Duration::new(5, 0))
                    .build_http())
        }
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors>
    {
        let res = self.https_client.get(URL.parse()?).await?;
        let body = aggregate(res).await?;
        let reader = body.reader();
        Ok(from_reader(reader)?)
    }
}
