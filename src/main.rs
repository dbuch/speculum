mod mirror;
mod mirrors;

use mirrors::Mirrors;

#[async_std::main]
async fn main() -> Result<(), surf::Exception> {
    let mirrors = Mirrors::fetch().await?;
    mirrors.rate().await;

    Ok(())
}
