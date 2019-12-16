use crate::mirror::Mirror;
use crate::mirror::Protocol;

use serde::Deserialize;
use chrono;
use chrono::prelude::*;
use itertools::Itertools;
use std::cmp::Ord;

static URL: &str = "https://www.archlinux.org/mirrors/status/json/";

#[derive(Clone, Deserialize, Debug)]
pub struct Mirrors
{
    cutoff: Option<u64>,
    last_check: Option<DateTime<Utc>>,
    num_checks: Option<u64>,
    check_frequency: Option<u64>,
    urls: Vec<Mirror>,
    version: u64,
}

impl Mirrors {
    pub async fn fetch() -> Result<Mirrors, surf::Exception>
    {
        let client = surf::Client::new();
        client.get(URL).recv_json().await
    }

    pub fn get(&self) -> impl Iterator<Item = &Mirror>
    {
        self.urls.iter()
            .sorted_by(|a, b| {
                b.completion_pct.partial_cmp(&a.completion_pct).unwrap_or(std::cmp::Ordering::Equal)
            })
            .sorted_by(|a, b| Ord::cmp(&b.last_sync, &a.last_sync))
            .filter(|&s| s.protocol == Protocol::Https)
    }
}

