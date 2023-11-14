use std::fs::read_to_string;

use anyhow::{Error, Ok, Result};

pub fn split_file(filename: &str) -> Result<Vec<Vec<String>>, Error> {
    let mut paragraphs = Vec::new();
    let mut paragraph = Vec::new();

    let content = read_to_string(filename)?;

    for mut line in content.lines() {
        line = line.trim();

        if line.is_empty() {
            paragraphs.push(paragraph.clone());
            paragraph.clear();
        } else {
            paragraph.push(line.into())
        }
    }

    if paragraph.len() > 0 {
        paragraphs.push(paragraph.clone());
    }

    paragraphs.retain(|element| !element.is_empty());

    Ok(paragraphs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_file() {
        let text = "./src/test-output/test-split.txt";

        let paragraphs = split_file(text).unwrap();

        assert_eq!(
            vec![
                vec!["test".to_string()],
                vec!["test".to_string()],
                vec!["test".to_string()],
                vec!["test".to_string()]
            ],
            paragraphs
        );
    }

    #[test]
    fn test_split_with_storm_warning() {
        let filename = "./src/test-output/storm-warning.txt";

        let splitted_file = split_file(filename).unwrap();

        let test_output = vec![
            vec!["WODL45 DWHA 030300".to_string()],
            vec![
                "STRONG WIND, GALE AND STORM WARNINGS FOR SEA AREAS:".to_string(),
                "GERMAN BIGHT, WESTERN AND SOUTHERN BALTIC.".to_string(),
            ],
            vec!["GERMAN BIGHT:".to_string(), "no warning.".to_string()],
            vec!["WESTERN BALTIC:".to_string(), "no warning.".to_string()],
            vec!["SOUTHERN BALTIC:".to_string(), "W 7 later.".to_string()],
            vec![
                "COASTAL AREA WARNINGS:".to_string(),
                "STARKWIND-, STURM- UND ORKANWARNUNGEN FUER DEUTSCHE KUESTEN.".to_string(),
            ],
            vec![
                "NR. 436".to_string(),
                "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg".to_string(),
                "fuer die deutsche Nordseekueste".to_string(),
                "herausgegeben am Montag, den 02.10.2023 um 23:50 Uhr GZ".to_string(),
            ],
            vec!["Ostfriesische Kueste:".to_string()],
            vec![
                "Suedwest 5 bis 6, dabei Boeen von 8 Beaufort, nordwestdrehend,".to_string(),
                "strichweise Gewitter.".to_string(),
                "Elbmuendung:".to_string(),
            ],
            vec![
                "Suedwest 5 bis 6, dabei Boeen von 8 Beaufort, nordwestdrehend,".to_string(),
                "strichweise Gewitter.".to_string(),
                "Seegebiet Helgoland:".to_string(),
            ],
            vec![
                "Suedwest 5 bis 6, dabei Boeen von 8 Beaufort, nordwestdrehend,".to_string(),
                "strichweise Gewitter.".to_string(),
                "Nordfriesische Kueste:".to_string(),
            ],
            vec![
                "Suedwest 5 bis 6, dabei Boeen von 8 Beaufort, nordwestdrehend,".to_string(),
                "strichweise Gewitter.".to_string(),
                "Elbe von Hamburg bis Cuxhaven:".to_string(),
            ],
            vec![
                "Boeen von 8 Beaufort aus Suedwest, nordwestdrehend, strichweise".to_string(),
                "Gewitter.".to_string(),
            ],
            vec![
                "NR. 406".to_string(),
                "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg".to_string(),
                "fuer die deutsche Ostseekueste".to_string(),
                "herausgegeben am Dienstag, den 03.10.2023 um 02:50 Uhr GZ".to_string(),
            ],
            vec!["Flensburg bis Fehmarn:".to_string()],
            vec![
                "Suedwest 5 bis 6,  dabei Boeen von 8 Beaufort, strichweise Gewitter.".to_string(),
                "oestlich Fehmarn bis Ruegen:".to_string(),
            ],
            vec![
                "Suedwest 5 bis 6,  dabei Boeen von 8 Beaufort, strichweise Gewitter.".to_string(),
                "oestlich Ruegen:".to_string(),
            ],
            vec!["Sued bis Suedwest 5 bis 6,  dabei Boeen von 7 Beaufort.".to_string()],
            vec!["Seewetterdienst Hamburg".to_string()],
        ];

        assert_eq!(splitted_file, test_output);
    }
}
