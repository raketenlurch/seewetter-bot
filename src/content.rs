use std::fs::{try_exists, File, OpenOptions};
use std::io::{prelude::*, Read};

use anyhow::{Context, Ok, Result};
use sha2::{Digest, Sha256};

/// Source: https://www.dwd.de/DE/leistungen/opendata/neuigkeiten/opendata_dez2020_01.html.
/// 7. LATEST-Dateien für Seewetterberichte verfügbar
const SEEWETTERBERICHT_NORD_OSTSEE: &str =
    "https://opendata.dwd.de/weather/maritime/forecast/german/FQEN50_EDZW_LATEST";

const FORECAST: &str = "forecast.txt";
const HASH: &str = "hash.txt";

// TODO: Error handling
pub async fn get_content() -> Result<(), anyhow::Error> {
    let is_modified = is_content_modified()
        .await
        .context("could not check if file contents changed AAA")?;

    dbg!(is_modified);

    if is_modified {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FORECAST)
            .context("could not open file")?;

        let response = reqwest::get(SEEWETTERBERICHT_NORD_OSTSEE)
            .await
            .context("could not fetch data from remote location")?
            .text()
            .await
            .context("could not get text from response")?;

        file.write_all(response.as_bytes())
            .context("could not write bytes to file 1")?;

        //dbg!(response);
    } else {
        println!("content has already been posted");
    }

    Ok(())
}

async fn is_content_modified() -> Result<bool, anyhow::Error> {
    let response = reqwest::get(SEEWETTERBERICHT_NORD_OSTSEE)
        .await
        .context("could not fetch data from remote location")?
        .text()
        .await
        .context("could not get text from response")?;

    if !try_exists(FORECAST).context("file does not exist")? {
        File::create_new(FORECAST).context("could not create file")?;
    }

    let mut file_forecast = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FORECAST)
        .context("could not create OpenOptions instance 1")?;

    file_forecast
        .write_all(response.as_bytes())
        .context("could not write bytes to file")?;

    let hash_new =
        compute_sha256_hash_of_file(FORECAST).context("could not get hash from file contents")?;
    let mut hash_old = String::new();

    if !try_exists(HASH).context("file does not exsit")? {
        File::create_new(HASH).context("could not create file")?;
    } else {
        let mut file_hash = OpenOptions::new()
            .read(true)
            .open(HASH)
            .context("could not create OpenOptions instance 2")?;

        file_hash
            .read_to_string(&mut hash_old)
            .context("could not get hash from file")?;
    }

    let mut is_content_modified = false;
    if hash_new != hash_old {
        is_content_modified = true;
    }

    let mut file_hash = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(HASH)
        .context("could not create OpenOptions instance 3")?;

    file_hash
        .write_all(hash_new.as_bytes())
        .context("could not write bytes to file")?;

    Ok(is_content_modified)
}

// Source: https://www.thorsten-hans.com/weekly-rust-trivia-compute-a-sha256-hash-of-a-file/
#[allow(unused)]
pub fn compute_sha256_hash_of_file(file: &str) -> Result<String, anyhow::Error> {
    let mut file = File::open(file).context("could not open file")?;

    let mut hasher = Sha256::new();

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file
            .read(&mut buffer)
            .context("could not read bytes from buffer")?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
