/*use std::fs::{metadata, File, OpenOptions};
use std::io::{copy, prelude::*};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Ok, Result};
use tokio::fs::{create_dir, remove_dir_all, remove_file};

pub async fn get_content() -> Result<(), anyhow::Error> {
    if let Err(_e) = File::create_new("filename.txt") {
        remove_file("filename.txt")
            .await
            .context("remove file...")?;
        File::create_new("filename.txt").context("failed to create file")?;
    }

    let mut path_str = String::from_str("./")?;
    path_str.push_str("filename.txt");
    let path = Path::new(&path_str);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let response = reqwest::get("https://opendata.dwd.de/weather/maritime/forecast/german/")
        .await
        .context("get response...")?
        .text()
        .await
        .context("get text from response")?;

    for line in response.lines() {
        if line.contains("LATEST") {
            if line.contains("COR") {
                continue;
            }

            let chars = line.chars();
            let prefix = chars.into_iter().take(29).collect::<String>();
            let suffix = &line[47..];

            let filename = line
                .strip_prefix(&prefix)
                .expect("could not strip prefix")
                .strip_suffix(suffix)
                .expect("could not strip suffix");

            file.write_all(filename.as_bytes())
                .context("could not write bytes to file")?;
            file.write_all(b"\n")
                .context("could not write bytes to file")?;
        }
    }

    let mut forecasts = String::new();
    File::open(path)
        .context("could not open file")?
        .read_to_string(&mut forecasts)
        .context("could not read file contents to string")?;

    if let Err(_e) = create_dir("./content").await {
        remove_dir_all("./content")
            .await
            .context("could not remove dir")?;
        create_dir("./content")
            .await
            .context("could not create dir")?;
    }

    for forecast in forecasts.lines() {
        let mut url =
            String::from_str("https://opendata.dwd.de/weather/maritime/forecast/german/")?;
        url.push_str(forecast);

        let content = reqwest::get(url)
            .await
            .context("get response...")?
            .text()
            .await
            .context("get text from response")?;

        let mut filename_str = String::from_str("./content/")?;
        filename_str.push_str(forecast);

        let mut filename = File::create_new(filename_str).context("could not create file")?;
        filename
            .write_all(content.as_bytes())
            .context("could not write bytes to files")?;
    }

    Ok(())
}*/

use std::fs::{metadata, read_to_string, try_exists, File, OpenOptions};
use std::io::{copy, prelude::*};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Ok, Result};
use reqwest::Client;
use tokio::fs::{create_dir, remove_dir_all, remove_file};

/// Die URLs zu den Seewetterberichten hardgecodet als Konstanten
/// Source: https://www.dwd.de/DE/leistungen/opendata/neuigkeiten/opendata_dez2020_01.html.
/// 7. LATEST-Dateien für Seewetterberichte verfügbar
const SEEWETTERBERICHT_NORD_OSTSEE: &str =
    "https://opendata.dwd.de/weather/maritime/forecast/german/FQEN50_EDZW_LATEST";

const DATE_FILE_PATH: &str = "date.txt";

// TODO: Error handling
pub async fn get_content() -> Result<(), anyhow::Error> {
    println!("get_content");

    Ok(())
}

async fn is_content_modified() -> Result<bool, anyhow::Error> {
    let client = Client::new();

    let response = client
        .get(SEEWETTERBERICHT_NORD_OSTSEE)
        .send()
        .await
        .context("could not fetch data from remote location")?;

    let last_modified_new = response
        .headers()
        .get("last-modified")
        .unwrap()
        .to_str()
        .context("could not convert last-modified value to string")?;

    if !try_exists(DATE_FILE_PATH).context("file does not exist")? {
        File::create_new(DATE_FILE_PATH).context("could not create file")?;
    }
    let last_modified_old =
        read_to_string(DATE_FILE_PATH).context("could not read file contents to string")?;

    if last_modified_new == last_modified_old {
        return Ok(false);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(DATE_FILE_PATH)
        .context("could not create OpenOptions instance")?;

    file.write_all(last_modified_new.as_bytes())
        .context("could not write bytes to file")?;

    Ok(true)
}
