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

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Mirrors
{
    cutoff: Option<u64>,
    last_check: Option<DateTime<Utc>>,
    num_checks: Option<u64>,
    check_frequency: Option<u64>,
    urls: Vec<Status>,
    version: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Status {
    url: Option<String>,
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

impl Mirrors {
    pub async fn fetch() -> Result<Mirrors, surf::Exception>
    {
        surf::get(URL).recv_json().await
    }

    pub fn get(&self) -> impl Iterator<Item = &Status>
    {
        self.urls.iter()
            .sorted_by(|a, b| {
                b.completion_pct.partial_cmp(&a.completion_pct).unwrap_or(std::cmp::Ordering::Equal)
            })
            .sorted_by(|a, b| Ord::cmp(&b.last_sync, &a.last_sync))
            .filter(|&s| s.protocol == Some("https".to_string()))
    }
}

impl Status {
    pub fn get_coredb_url(&self) -> Option<String>
    {
        if let Some(mut url) = self.url.clone()
        {
            url.push_str("core/os/x86_64/core.db");
            return Some(url);
        }
        None
    }
}

#[async_std::main]
async fn main() -> Result<(), surf::Exception> {
    let mirrors = Mirrors::fetch().await?;
    let latest = mirrors.get();
    let mut tasks: Vec<JoinHandle<(String, f64)>> = Vec::new();

    for status in latest.take(20) {
        if let Some(db_url) = status.get_coredb_url()
        {
            let url = db_url.clone();
            tasks.push(
                task::spawn(async move {
                    let now = Instant::now();
                    let mut res = surf::get(db_url).await.unwrap();
                    let n_bytes = res.body_bytes().await.unwrap();
                    let elapsed = now.elapsed();
                    (url, n_bytes.len() as f64 / 1_000.0 / elapsed.as_secs_f64())
                })
            );
        }
    }

    for task in tasks {
        let (url, rate) = task.await;
        println!("Rate: {0: >8.1} KiB/s [{1}]", rate, url);
    }

    Ok(())
}
