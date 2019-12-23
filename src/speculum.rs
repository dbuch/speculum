use super::Mirrors;
use bytes::buf::BufExt as _;
use hyper::{
    body::{aggregate, Body},
    client::Client,
    client::HttpConnector,
};
use hyper_tls::HttpsConnector;
use serde_json::from_reader;
use std::rc::Rc;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Speculum {
    https_client: Rc<hyper::Client<HttpsConnector<HttpConnector>, Body>>,
    #[allow(dead_code)] // Not all mirrors uses https
    http_client: Rc<hyper::Client<HttpConnector, Body>>,
}

impl Speculum {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        Speculum {
            https_client: Rc::new(
                Client::builder()
                    .keep_alive_timeout(std::time::Duration::new(5, 0))
                    .build(https),
            ),
            http_client: Rc::new(
                Client::builder()
                    .keep_alive_timeout(std::time::Duration::new(5, 0))
                    .build_http(),
            ),
        }
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors> {
        let res = self.https_client.get(URL.parse()?).await?;
        let body = aggregate(res).await?;
        let reader = body.reader();
        Ok(from_reader(reader)?)
    }
}
