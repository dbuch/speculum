mod mirror;
mod mirrors;
mod protocols;

use anyhow::Result;
use reqwest::Client;

pub use mirror::Mirror;
pub use mirrors::Mirrors;
pub use protocols::Protocols;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

pub struct Speculum {
    client: reqwest::Client,
}

impl Speculum {
    pub fn new() -> Self {
        Speculum {
            client: Client::new(),
        }
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors> {
        let mut mirrors: Mirrors = self.client.get(URL).send().await?.json::<Mirrors>().await?;
        mirrors
            .get_urls_mut()
            .retain(|url| url.score.is_some() && url.active.unwrap());
        Ok(mirrors)
    }
}
