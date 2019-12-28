use env_logger;
use itertools::Itertools;
use log::*;
use tokio::fs::OpenOptions;
use tokio::prelude::*;
use speculum::{Cli, Speculum};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Cli::initialize();

    match options.verbose {
        1 => std::env::set_var("RUST_LOG", "speculum=info"),
        2 => std::env::set_var("RUST_LOG", "trace"),
        3 => std::env::set_var("RUST_LOG", "warn"),
        4 => std::env::set_var("RUST_LOG", "error"),
        5 => std::env::set_var("RUST_LOG", "debug"),
        _ => {}
    }

    env_logger::init();

    let speculum = Speculum::new();
    let mirrors = speculum.fetch_mirrors().await?;

    info!("Mirrors has been fetched!");

    let fetched: String = mirrors
        .into_iter()
        .filter(|mirror| mirror.score.is_some() && mirror.url.is_some())
        .filter(|mirror| {
            mirror.protocol.intercects(options.filters.protocols)
        })
        .sorted_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .map(|m| m.to_string())
        .join("\n");

    let mut mirrorlist = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(options.optional.save)
        .await?;

    mirrorlist.write(fetched.as_bytes()).await?;

    Ok(())
}
