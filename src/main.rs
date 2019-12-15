use surf;
use chrono;
use chrono::prelude::*;
use serde::{
    Serialize,
    Deserialize,
};
use itertools::Itertools;
use std::cmp::Ord;
use std::time::Instant;
use async_std::task;
use async_std::task::JoinHandle;
use async_macros::Join;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

enum Protocol
{
    Https,
    Http,
    Rsync,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(bound(deserialize = "Mirrors<'a>: Deserialize<'a>"))]
struct Mirrors<'a>
{
    cutoff: Option<u64>,
    last_check: Option<DateTime<Utc>>,
    num_checks: Option<u64>,
    check_frequency: Option<u64>,
    #[serde(bound(deserialize = "Vec<Mirror<'a>>: Deserialize<'de>"))]
    urls: Vec<Mirror<'a>>,
    version: u64,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(bound(deserialize = "Mirror<'de>: Deserialize<'de>"))]
struct Mirror<'a> {
    url: &'a str,
    protocol: Option<String>,
    last_sync: Option<DateTime<Utc>>,
    completion_pct: f64,
    delay: Option<u64>,
    duration_avg: Option<f64>,
    duration_stddev: Option<f64>,
    score: Option<f64>,
    active: Option<bool>,
    country: Option<String>,
    country_code: Option<String>,
    isos: Option<bool>,
    ipv4: bool,
    ipv6: bool,
    details: Option<String>
}

impl<'a> Mirrors<'a> {
    pub async fn fetch() -> Result<Mirrors<'a>, surf::Exception>
    {
        let client = surf::Client::new();
        client.get(URL).recv_json().await?
    }

    pub fn get(&self) -> impl Iterator<Item = &Mirror>
    {
        self.urls.iter()
            .sorted_by(|a, b| {
                b.completion_pct.partial_cmp(&a.completion_pct).unwrap_or(std::cmp::Ordering::Equal)
            })
            .sorted_by(|a, b| Ord::cmp(&b.last_sync, &a.last_sync))
            .filter(|&s| s.protocol == Some("https".to_string()))
    }
}

impl<'a> Mirror<'a> {
    pub fn get_coredb_url(&self) -> String
    {
        let string = self.url.to_string();
        string.push_str("core/os/x86_64/core.db");
        string
    }
}

#[async_std::main]
async fn main() -> Result<(), surf::Exception> {
    let mirrors = Mirrors::fetch().await?;
    let latest = mirrors.get();
    let mut tasks: Vec<JoinHandle<(&str, f64)>> = Vec::new();

    for mirror in latest.take(20) {
        let db_url = mirror.get_coredb_url();
        tasks.push(
            task::spawn(async move {
                let now = Instant::now();
                let mut res = surf::get(db_url).await.unwrap();
                let n_bytes = res.body_bytes().await.unwrap();
                let elapsed = now.elapsed();
                (mirror.url, n_bytes.len() as f64 / 1_000.0 / elapsed.as_secs_f64())
            })
        );
    }

    for task in tasks {
        let (url, rate) = task.await;
        println!("Rate: {0: >8.1} KiB/s [{1}]", rate, url);
    }

    Ok(())
}
