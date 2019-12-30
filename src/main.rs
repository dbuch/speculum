use env_logger;
use speculum::{Cli, Speculum, Mirrors};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
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
    let mut mirrors: Mirrors = speculum.fetch_mirrors().await?;

    mirrors
        .filter_protocols(options.filters.protocols)
        .order_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .take(options.filters.latest)
        .save(options.optional.save)
        .await?;

    Ok(())
}
