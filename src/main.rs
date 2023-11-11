#![feature(file_create_new)]
#![feature(fs_try_exists)]

use crate::content::get_content;

mod content;
mod parse_storm_warning;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_content().await?;

    Ok(())
}
