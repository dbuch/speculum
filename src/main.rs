use env_logger;
use log::*;
use tokio::fs::OpenOptions;
use tokio::prelude::*;
use itertools::Itertools;
use speculum::speculum::Speculum;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const MIRRORLIST: &str = "/etc/pacman.d/mirrorlist";

fn check_root() {
    let is_root = users::get_current_uid() == 0;

    if !is_root
    {
        eprintln!("This program should be run as root!");
        std::process::exit(1)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    check_root();

    env_logger::init();

    let speculum = Speculum::new();
    let mirrors = speculum.fetch_mirrors().await?;

    info!("Mirrors has been fetched!");

    let fetched: String =
        mirrors
        .into_iter()
        .filter(|mirror| mirror.protocol.is_some() && mirror.protocol.as_ref().unwrap().starts_with("http"))
        .filter(|mirror| mirror.score.is_some())
        .sorted_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .take(20)
        .map(|m| m.to_string())
        .join("\n");

    let mut mirrorlist = 
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(MIRRORLIST).await?;
    info!("writing mirror list!");
    mirrorlist.write(fetched.as_bytes()).await?;

    Ok(())
}
