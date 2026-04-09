mod adapter;
mod core;
mod domain;

use core::CoreBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let core = CoreBuilder::new()?
        .registry()
        .await?
        .services()
        .await?
        .gateway()
        .await?
        .build()?;

    core.run().await
}
