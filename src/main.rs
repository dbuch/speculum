use env_logger;
use log::*;
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
    let mut mirrors = speculum.fetch_mirrors().await?;

    info!("Mirrors has been fetched!");

    mirrors
        .order_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .filter_protocols(options.filters.protocols)
        .take(options.filters.latest)
        .save(options.optional.save)
        .await?;

    //let fetched: String = mirrors
    //.into_iter()
    //.filter(|mirror| mirror.score.is_some() && mirror.url.is_some())
    //.filter(|mirror| mirror.protocol.intercects(options.filters.protocols))
    //.sorted_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
    //.map(|m| m.to_string())
    //.join("\n");

    Ok(())
}
