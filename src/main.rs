#![feature(file_create_new)]
use tracing::debug;
use tracing_subscriber;

use crate::content::get_content;

mod content;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // TODO: Fix tracing debug output
    debug!("test");

    get_content().await?;

    Ok(())
}
