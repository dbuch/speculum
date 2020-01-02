use speculum::{Cli, Result, Speculum};

#[tokio::main]
async fn main() -> Result<()> {
    let options = Cli::initialize()?;

    log::trace!("Started");

    let speculum = Speculum::new();

    speculum
        .fetch_mirrors().await?
        .filter_protocols(options.filters.protocols)
        .order_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .take(options.filters.latest)
        .save(options.optional.save)
        .await?;


    Ok(())
}
