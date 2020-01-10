use log::*;
use speculum::{Cli, Mirrors, Result, Speculum};

#[tokio::main]
async fn main() -> Result<()> {
    let options = Cli::initialize()?;

    let speculum = Speculum::new()?;
    let mut mirrors: Mirrors = speculum.fetch_mirrors().await?;
    let mut stdout = tokio::io::stdout();

    mirrors
        .filter_protocols(options.filters.protocols)
        .order_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap()
                .then(a.last_sync.cmp(&b.last_sync))
        })
        .take(options.filters.latest)
        .write(&mut stdout)
        .await?;

    Ok(())
}
