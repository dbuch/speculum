mod speculum;
mod mirrors;
mod mirror;

use speculum::Speculum;
use itertools::Itertools;
use users::get_current_uid;
use tokio::fs;
use tokio::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const MIRRORLIST: &str = "/etc/pacman.d/mirrorlist";

#[tokio::main]
async fn main() -> Result<()> {

    if get_current_uid() != 0
    {
        eprintln!("This program should be run as root!");
        return Ok(());
    }

    let speculum = Speculum::new();
    let mirrors = speculum.fetch_mirrors().await?;

    let fetched: String =
        mirrors
        .into_iter()
        .sorted_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .filter(|mirror| mirror.score.is_some())
        .filter(|mirror| mirror.protocol.as_ref().unwrap().starts_with("http"))
        .take(20)
        .sorted_by(|a, b| a.last_sync.cmp(&b.last_sync))
        .map(|m| m.to_string())
        .join("\n");

    let mut mirrorlist = fs::File::open(MIRRORLIST).await?;
    mirrorlist.write_all(fetched.as_bytes()).await?;

    /*
    mirrors.urls
        .iter()
        .filter(|http| http.protocol == mirror::Protocol::Https)
        .sorted_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .for_each(|mirror| {
            let url = mirror.url.clone();
            let db_url = mirror.get_coredb_url();
            tasks.push(task::spawn(async move {
                if let Ok(mut res) = surf::get(db_url).await
                {
                    let now = Instant::now();
                    if let Ok(n_bytes) = res.body_bytes().await
                    {
                        let elapsed = now.elapsed();
                        println!("{}", elapsed.as_secs_f64());
                        return Some((url, n_bytes.len() as f64 / elapsed.as_secs_f64()));
                    }
                    return None;
                }
                None
            }));
        });

    let mut count = 0u64;
    println!("Rating {} / {}", count, n_mirrors);
    let mut res = Vec::<(String, f64)>::new();
    for task in tasks {
        if let Some((url, rate)) = task.await
        {
            count += 1;
            res.push((url, rate));
        }
    }
    println!("Done Rated {} mirrors", &res.len());

    res.iter().sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
        .for_each(|(url, rate)| println!("{} {}", rate, url));
    //mirrors.rate().await;

    */
    Ok(())
}
