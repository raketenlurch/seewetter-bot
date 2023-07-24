/*use std::fs::{metadata, File, OpenOptions};
use std::io::{copy, prelude::*};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Ok, Result};
use tokio::fs::{create_dir, remove_dir, remove_file};

pub async fn get_content() -> Result<()> {
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
        .context("get text from reponse")?;

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
                .expect("could not strip  prefix")
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
        remove_dir("./content")
            .await
            .context("could not remove dir")?;
        create_dir("./content")
            .await
            .context("coudl not create dir")?;
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
        filename.write_all(content.as_bytes())?;
    }

    Ok(())
}*/

use std::fs::{metadata, File, OpenOptions};
use std::io::{copy, prelude::*};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Ok, Result};
use tokio::fs::{create_dir, remove_dir_all, remove_file};
use tracing::debug;

pub async fn get_content() -> Result<(), anyhow::Error> {
    if let Err(_e) = File::create_new("filename.txt") {
        remove_file("filename.txt")
            .await
            .context("remove file...")?;
        File::create_new("filename.txt").context("failed to create file")?;
        debug!("create file...");
    }

    let mut path_str = String::from_str("./")?;
    path_str.push_str("filename.txt");
    let path = Path::new(&path_str);
    debug!("created file at: {:#?}", path);

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
    debug!("got text {:#?} from location \"https://opendata.dwd.de/weather/maritime/forecast/german/\"", response);

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
            debug!("bytes are written to file");
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
        debug!("create dir...");
    }

    for forecast in forecasts.lines() {
        let mut url =
            String::from_str("https://opendata.dwd.de/weather/maritime/forecast/german/")?;
        url.push_str(forecast);
        debug!("get content from location {:#?}", url);

        let content = reqwest::get(url)
            .await
            .context("get response...")?
            .text()
            .await
            .context("get text from response")?;

        let mut filename_str = String::from_str("./content/")?;
        filename_str.push_str(forecast);
        debug!("write contents to {:#?}", filename_str);

        let mut filename = File::create_new(filename_str).context("could not create file")?;
        filename
            .write_all(content.as_bytes())
            .context("could not write bytes to files")?;
        debug!("bytes are written to file");
    }

    Ok(())
}

fn remove_empty_lines(file: File) {}
