use anyhow::Result;
use speculum::{Cli, Mirrors, Speculum};
use tokio::fs::{OpenOptions, File};
use std::path::PathBuf;

async fn save_file(path: PathBuf) -> Result<File>
{
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .await?;

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
        .write(&mut file)
        .await?;

    Ok(())
}
