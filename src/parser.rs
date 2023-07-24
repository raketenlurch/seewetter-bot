// TODO: Translate to English

use std::path::Path;

use anyhow::{Context, Ok, Result};
use tokio::{
    fs::{read_to_string, File},
    io::AsyncReadExt,
};

struct Ort {
    englischer_kanal_westteil: String,
    englischer_kanal_ostteil: String,
    ijsselmeer: String,
    deutsche_bucht: String,
    suedwestliche_nordsee: String,
    fischer: String,
    dogger: String,
    forties: String,
    viking: String,
    utsira: String,
    skagerrak: String,
    kattegat: String,
    belte_und_sund: String,
    westliche_ostsee: String,
    suedliche_ostsee: String,
    boddengewaesser_ost: String,
    suedoestliche_ostsee: String,
    zentrale_ostsee: String,
    noerdliche_ostsee: String,
    rigaischer_meerbusen: String,
}

struct Vorhersage {
    titel: String,
    datum: String,
    starkwind: Vec<String>,
    wetterlage: String,
    vorhersage: Ort,
    aussichten: Ort,
    windvorhersage: String,
    luftdruck: String,
}

impl Ort {
    fn new() -> Self {
        Self {
            englischer_kanal_westteil: String::new(),
            englischer_kanal_ostteil: String::new(),
            ijsselmeer: String::new(),
            deutsche_bucht: String::new(),
            suedwestliche_nordsee: String::new(),
            fischer: String::new(),
            dogger: String::new(),
            forties: String::new(),
            viking: String::new(),
            utsira: String::new(),
            skagerrak: String::new(),
            kattegat: String::new(),
            belte_und_sund: String::new(),
            westliche_ostsee: String::new(),
            suedliche_ostsee: String::new(),
            boddengewaesser_ost: String::new(),
            suedoestliche_ostsee: String::new(),
            zentrale_ostsee: String::new(),
            noerdliche_ostsee: String::new(),
            rigaischer_meerbusen: String::new(),
        }
    }
}

impl Vorhersage {
    fn new() -> Self {
        Self {
            titel: String::new(),
            datum: String::new(),
            starkwind: Vec::new(),
            wetterlage: String::new(),
            vorhersage: Ort::new(),
            aussichten: Ort::new(),
            windvorhersage: String::new(),
            luftdruck: String::new(),
        }
    }

    /*async fn parse(directory: &Path) -> Result<Self, anyhow::Error> {
        let files = Vec::new();
        let mut buffer: Vec<u8> = Vec::new();

        for path in directory {
            if let Some(path) = path.to_str() {
                files.push(path);
            }
        }

        for path in files {
            let mut file = File::open(path).await.context("could not open file")?;
            file.read_to_end(&mut buffer).await;

            let content = read_to_string(&mut buffer)
                .await
                .context("could not read file content to string")?;
        }

        Ok(Self {
            titel: (),
            datum: (),
            starkwind: (),
            wetterlage: (),
            vorhersage: (),
            aussichten: (),
            windvorhersage: (),
            luftdruck: (),
        })
    }*/

    async fn parse(directory: &Path) -> Result<Self, anyhow::Error> {
        let contents = read_to_string(directory)
            .await
            .context("could not read file content to string")?;

        for segment in contents.split("\n\r\n\r\n\r") {}

        Ok(Self {
            titel: (),
            datum: (),
            starkwind: (),
            wetterlage: (),
            vorhersage: (),
            aussichten: (),
            windvorhersage: (),
            luftdruck: (),
        })
    }
}
