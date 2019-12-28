mod protocols;
mod mirrors;
mod mirror;

use reqwest::Client;

pub use mirrors::Mirrors;
pub use mirror::Mirror;
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

    pub async fn fetch_mirrors(&self) -> Result<Mirrors, reqwest::Error> {
        self.client.get(URL).send().await?.json::<Mirrors>().await
    }
}
