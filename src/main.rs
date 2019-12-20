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

    Ok(())
}
