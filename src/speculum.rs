use super::Mirrors;
use reqwest::Client;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

pub struct Speculum {
    client: reqwest::Client
}

impl Speculum {
    pub fn new() -> Self {
        Speculum {
            client: Client::new()
        }
    }

    pub async fn fetch_mirrors(&self) -> Result<Mirrors, reqwest::Error> {
        self.client.get(URL).send().await?.json::<Mirrors>().await
    }
}
