mod mirror;
mod mirrors;
mod protocols;

use reqwest::Client;

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;

pub use mirror::Mirror;
pub use mirrors::Mirrors;
pub use protocols::Protocols;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

pub struct Speculum {
    client: reqwest::Client,
}

impl Default for Speculum
{
    fn default() -> Self
    {
        Speculum {
            client: Client::new()
        }
    }
}

impl Speculum {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors> {
        let mut mirrors: Mirrors = self.client.get(URL).send().await?.json::<Mirrors>().await?;
        mirrors
            .get_urls_mut()
            .retain(|url| url.score.is_some() && url.active.unwrap());
        Ok(mirrors)
    }
}
