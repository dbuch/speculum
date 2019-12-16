mod mirror;
mod mirrors;

use crate::mirrors::Mirrors;
use surf;
use std::time::Instant;
use async_std::{
    task::{
        self,
        JoinHandle
    }
};

#[async_std::main]
async fn main() -> Result<(), surf::Exception> {
    let mirrors = Mirrors::fetch().await?;
    let latest = mirrors.get();
    let mut tasks: Vec<JoinHandle<(String, f64)>> = Vec::new();

    for mirror in latest.take(20) {
        let url = mirror.url.clone().unwrap();
        let db_url = mirror.get_coredb_url();
        tasks.push(
            task::spawn(async move {
                let now = Instant::now();
                let mut res = surf::get(db_url).await.unwrap();
                let n_bytes = res.body_bytes().await.unwrap();
                let elapsed = now.elapsed();
                (url, n_bytes.len() as f64 / elapsed.as_secs_f64())
            })
        );
    }

    for task in tasks {
        let (url, rate) = task.await;
        println!("Rate: {:6.2} {}", rate / 1000.0, url);
    }

    Ok(())
}
