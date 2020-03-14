use anyhow::{Context, Result};
use speculum::{Cli, Mirrors, Speculum};
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};

async fn save_file(path: PathBuf) -> Result<File> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .await
        .context(format!("Cannot write mirrors to: \"{}\"", path.to_string_lossy()))?;

    Ok(file)
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Cli::initialize()?;

    let speculum = Speculum::new()?;

    let mut mirrors: Mirrors = speculum.fetch_mirrors().await?;
    let mut file = save_file(options.optional.write).await?;

    mirrors
        .filter_protocols(options.filters.protocols)
        .order_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .take(options.filters.latest)
        .order_by(|a, b| a.last_sync.cmp(&b.last_sync))
        .take(10)
        .rate_all().await?
        .order_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap())
        .write(&mut file)
        .await?;

    Ok(())
}
