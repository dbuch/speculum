use speculum::{Cli, Mirrors, Result, Speculum};

#[tokio::main]
async fn main() -> Result<()> {
    let options = Cli::initialize()?;

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
