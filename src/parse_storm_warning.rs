use nom::{bytes::complete::take_until, IResult};

#[derive(Debug, PartialEq)]
struct StormWarning {
    english: English,
    german: German,
    source: String,
}

#[derive(Debug, PartialEq)]
struct English {
    title: String,
    german_bight: String,
    western_baltic: String,
    southern_baltic: String,
    coastal_area_warning: String,
}

#[derive(Debug, PartialEq)]
struct German {
    north_sea_coast: HighWindWarningGermanNorthSeaCoast,
    baltic_coast: HighWindWarningGermanBalticCoast,
}

#[derive(Debug, PartialEq)]
struct HighWindWarningGermanNorthSeaCoast {
    number: String,
    title: String,
    east_frisian_coast: String,
    elbe_estuary: String,
    sea_area_helgoland: String,
    north_frisian_coast: String,
    elbe_from_hamburg_to_cuxhaven: String,
}

#[derive(Debug, PartialEq)]
struct HighWindWarningGermanBalticCoast {
    number: String,
    title: String,
    flensburg_to_fehmarn: String,
    east_of_fehmarn_to_ruegen: String,
    east_from_ruegen: String,
}

impl StormWarning {}

impl English {}

impl German {
    fn new() -> Self {
        Self {
            north_sea_coast: HighWindWarningGermanNorthSeaCoast::new(),
            baltic_coast: HighWindWarningGermanBalticCoast::new(),
        }
    }

    fn parse(
        north_sea_coast: HighWindWarningGermanNorthSeaCoast,
        baltic_coast: HighWindWarningGermanBalticCoast,
    ) -> Self {
        Self {
            north_sea_coast,
            baltic_coast,
        }
    }
}

impl HighWindWarningGermanNorthSeaCoast {
    pub fn new() -> Self {
        Self {
            number: String::new(),
            title: String::new(),
            east_frisian_coast: String::new(),
            elbe_estuary: String::new(),
            sea_area_helgoland: String::new(),
            north_frisian_coast: String::new(),
            elbe_from_hamburg_to_cuxhaven: String::new(),
        }
    }

    fn get_number(text: &str) -> IResult<&str, &str> {
        take_until("Amtliche")(text)
    }

    fn get_title(text: &str) -> IResult<&str, &str> {
        take_until("Ostfriesische Kueste:")(text)
    }

    fn get_east_frisian_coast(text: &str) -> IResult<&str, &str> {
        take_until("Elbmuendung:")(text)
    }

    fn get_elbe_estuary(text: &str) -> IResult<&str, &str> {
        take_until("Seegebiet Helgoland:")(text)
    }

    fn get_sea_area_helgoland(text: &str) -> IResult<&str, &str> {
        take_until("Nordfriesische Kueste:")(text)
    }

    fn get_north_frisian_coast(text: &str) -> IResult<&str, &str> {
        take_until("Elbe von Hamburg bis Cuxhaven:")(text)
    }

    fn get_elbe_from_hamburg_to_cuxhaven(text: &str) -> IResult<&str, &str> {
        take_until(".")(text)
    }

    fn parse(filename: &str) -> Self {
        let number_with_newline = Self::get_number(filename).unwrap();
        let mut number = number_with_newline.1.replace("\n", " ");
        if number.ends_with(" ") {
            number = number.trim().to_string();
        }

        let title_with_newline = Self::get_title(number_with_newline.0).unwrap();
        let mut title = title_with_newline.1.replace("\n", " ");
        if title.ends_with(" ") {
            title = title.trim().to_string();
        }

        let east_frisian_coast_with_newline =
            Self::get_east_frisian_coast(title_with_newline.0).unwrap();
        let mut east_frisian_coast = east_frisian_coast_with_newline.1.replace("\n", " ");
        if east_frisian_coast.ends_with(" ") {
            east_frisian_coast = east_frisian_coast.trim().to_string();
        }

        let elbe_estuary_with_newline =
            Self::get_elbe_estuary(east_frisian_coast_with_newline.0).unwrap();
        let mut elbe_estuary = elbe_estuary_with_newline.1.replace("\n", " ");
        if elbe_estuary.ends_with(" ") {
            elbe_estuary = elbe_estuary.trim().to_string();
        }

        let sea_area_helgoland_with_newline =
            Self::get_sea_area_helgoland(elbe_estuary_with_newline.0).unwrap();
        let mut sea_area_helgoland = sea_area_helgoland_with_newline.1.replace("\n", " ");
        if sea_area_helgoland.ends_with(" ") {
            sea_area_helgoland = sea_area_helgoland.trim().to_string();
        }

        let north_frisian_coast_with_newline =
            Self::get_north_frisian_coast(sea_area_helgoland_with_newline.0).unwrap();
        let mut north_frisian_coast = north_frisian_coast_with_newline.1.replace("\n", " ");
        if north_frisian_coast.ends_with(" ") {
            north_frisian_coast = north_frisian_coast.trim().to_string();
        }

        let elbe_from_hamburg_to_cuxhaven_with_newline =
            Self::get_elbe_from_hamburg_to_cuxhaven(north_frisian_coast_with_newline.0).unwrap();
        let mut elbe_from_hamburg_to_cuxhaven = elbe_from_hamburg_to_cuxhaven_with_newline
            .1
            .replace("\n", " ");
        if elbe_from_hamburg_to_cuxhaven.ends_with(" ") {
            elbe_from_hamburg_to_cuxhaven = elbe_from_hamburg_to_cuxhaven.trim().to_string();
        }

        Self {
            number,
            title,
            east_frisian_coast,
            elbe_estuary,
            sea_area_helgoland,
            north_frisian_coast,
            elbe_from_hamburg_to_cuxhaven,
        }
    }
}

impl HighWindWarningGermanBalticCoast {
    pub fn new() -> Self {
        Self {
            number: String::new(),
            title: String::new(),
            flensburg_to_fehmarn: String::new(),
            east_of_fehmarn_to_ruegen: String::new(),
            east_from_ruegen: String::new(),
        }
    }

    fn get_number(text: &str) -> IResult<&str, &str> {
        take_until("Amtliche")(text)
    }

    fn get_title(text: &str) -> IResult<&str, &str> {
        take_until("Flensburg")(text)
    }

    fn get_flensburg_to_fehmarn(text: &str) -> IResult<&str, &str> {
        take_until("oestlich Fehmarn")(text)
    }

    fn get_oestlich_fehmarn_to_ruegen(text: &str) -> IResult<&str, &str> {
        take_until("oestlich Ruegen")(text)
    }

    fn get_oestlich_ruegen(text: &str) -> IResult<&str, &str> {
        take_until(".")(text)
    }

    fn parse(filename: &str) -> Self {
        let number_with_newline = Self::get_number(filename).unwrap();
        let mut number = number_with_newline.1.replace("\r", "").replace("\n", " ");
        if number.ends_with(" ") {
            number = number.trim().to_string();
        }

        let title_with_newline = Self::get_title(number_with_newline.0).unwrap();
        let mut title = title_with_newline.1.replace("\r", "").replace("\n", " ");
        if title.ends_with(" ") {
            title = title.trim().to_string();
        }

        let flensburg_to_fehmarn_with_newline =
            Self::get_flensburg_to_fehmarn(title_with_newline.0).unwrap();
        let mut flensburg_to_fehmarn = flensburg_to_fehmarn_with_newline
            .1
            .replace("\r", "")
            .replace("\n", " ");
        if flensburg_to_fehmarn.ends_with(" ") {
            flensburg_to_fehmarn = flensburg_to_fehmarn.trim().to_string();
        }

        let east_of_fehmarn_to_ruegen_with_newline =
            Self::get_oestlich_fehmarn_to_ruegen(flensburg_to_fehmarn_with_newline.0).unwrap();
        let mut east_of_fehmarn_to_ruegen = east_of_fehmarn_to_ruegen_with_newline
            .1
            .replace("\r", "")
            .replace("\n", " ");
        if east_of_fehmarn_to_ruegen.ends_with(" ") {
            east_of_fehmarn_to_ruegen = east_of_fehmarn_to_ruegen.trim().to_string();
        }

        let east_from_ruegen_with_newline =
            Self::get_oestlich_ruegen(east_of_fehmarn_to_ruegen_with_newline.0).unwrap();
        let mut east_from_ruegen = east_from_ruegen_with_newline
            .1
            .replace("\r", "")
            .replace("\n", " ");
        if east_from_ruegen.ends_with(" ") {
            east_from_ruegen = east_from_ruegen.trim().to_string();
        }

        Self {
            number,
            title,
            flensburg_to_fehmarn,
            east_of_fehmarn_to_ruegen,
            east_from_ruegen,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn high_wind_warning_german_north_sea_coast() {
        let filename = "./src/test-output/high-wind-warning-german-north-sea-coast.txt";
        let content = read_to_string(filename).unwrap();

        let high_wind_warning_north_sea_coast = HighWindWarningGermanNorthSeaCoast::parse(&content);

        let mut test_output_german_north_sea_coast = HighWindWarningGermanNorthSeaCoast::new();

        test_output_german_north_sea_coast.number = "NR. 437".to_string();
        test_output_german_north_sea_coast.title = "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg fuer die deutsche Nordseekueste herausgegeben am Mittwoch, den 04.10.2023 um 08:30 Uhr GZ"
            .to_string();
        test_output_german_north_sea_coast.east_frisian_coast =
            "Ostfriesische Kueste:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output_german_north_sea_coast.elbe_estuary =
            "Elbmuendung:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort.".to_string();
        test_output_german_north_sea_coast.sea_area_helgoland =
            "Seegebiet Helgoland:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output_german_north_sea_coast.north_frisian_coast =
            "Nordfriesische Kueste:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output_german_north_sea_coast.elbe_from_hamburg_to_cuxhaven = "Elbe von Hamburg bis Cuxhaven:  Westteil Suedwest bis West 5 bis 6, dabei Boeen von 8 Beaufort"
            .to_string();

        assert_eq!(
            high_wind_warning_north_sea_coast,
            test_output_german_north_sea_coast
        );
    }

    #[test]
    fn high_wind_warning_german_baltic_coast() {
        let filename = "./src/test-output/high-wind-warning-german-baltic-coast.txt";
        let content = read_to_string(filename).unwrap();

        let high_wind_warning_german_baltic_coast =
            HighWindWarningGermanBalticCoast::parse(&content);

        let mut test_output_german_baltic_coast = HighWindWarningGermanBalticCoast::new();

        test_output_german_baltic_coast.number = "NR. 411".to_string();
        test_output_german_baltic_coast.title = "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg fuer die deutsche Ostseekueste herausgegeben am Mittwoch, den 04.10.2023 um 08:30 Uhr GZ"
            .to_string();
        test_output_german_baltic_coast.flensburg_to_fehmarn =
            "Flensburg bis Fehmarn: West bis Suedwest 5 bis 6, dabei Boeen von 8 Beaufort."
                .to_string();
        test_output_german_baltic_coast.east_of_fehmarn_to_ruegen =
            "oestlich Fehmarn bis Ruegen: West bis Suedwest 5 bis 6, dabei Boeen von 8 Beaufort."
                .to_string();
        test_output_german_baltic_coast.east_from_ruegen =
            "oestlich Ruegen: West bis Suedwest 5 bis 6, dabei Boeen von 8 Beaufort".to_string();

        assert_eq!(
            high_wind_warning_german_baltic_coast,
            test_output_german_baltic_coast
        );
    }

    #[test]
    fn test_parse_german() {
        let filename_high_wind_warning_german_north_sea_coast =
            "./src/test-output/high-wind-warning-german-north-sea-coast.txt";
        let filename_high_wind_warning_german_baltic_coast =
            "./src/test-output/high-wind-warning-german-baltic-coast.txt";

        let content_high_wind_warning_german_north_sea_coast =
            read_to_string(filename_high_wind_warning_german_north_sea_coast).unwrap();
        let content_high_wind_warning_german_baltic_coast =
            read_to_string(filename_high_wind_warning_german_baltic_coast).unwrap();

        let high_wind_warning_german_north_sea_coast = HighWindWarningGermanNorthSeaCoast::parse(
            &content_high_wind_warning_german_north_sea_coast,
        );
        let high_wind_warning_german_baltic_coast =
            HighWindWarningGermanBalticCoast::parse(&content_high_wind_warning_german_baltic_coast);

        let german = German::parse(
            high_wind_warning_german_north_sea_coast,
            high_wind_warning_german_baltic_coast,
        );

        let mut test_output_german = German::new();
        let mut test_output_german_north_sea_coast = HighWindWarningGermanNorthSeaCoast::new();
        let mut test_output_german_baltic_coast = HighWindWarningGermanBalticCoast::new();

        test_output_german_north_sea_coast.number = "NR. 437".to_string();
        test_output_german_north_sea_coast.title = "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg fuer die deutsche Nordseekueste herausgegeben am Mittwoch, den 04.10.2023 um 08:30 Uhr GZ".to_string();
        test_output_german_north_sea_coast.east_frisian_coast =
            "Ostfriesische Kueste:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output_german_north_sea_coast.elbe_estuary =
            "Elbmuendung:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort.".to_string();
        test_output_german_north_sea_coast.sea_area_helgoland =
            "Seegebiet Helgoland:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output_german_north_sea_coast.north_frisian_coast =
            "Nordfriesische Kueste:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output_german_north_sea_coast.elbe_from_hamburg_to_cuxhaven = "Elbe von Hamburg bis Cuxhaven:  Westteil Suedwest bis West 5 bis 6, dabei Boeen von 8 Beaufort".to_string();

        test_output_german_baltic_coast.number = "NR. 411".to_string();
        test_output_german_baltic_coast.title = "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg fuer die deutsche Ostseekueste herausgegeben am Mittwoch, den 04.10.2023 um 08:30 Uhr GZ".to_string();
        test_output_german_baltic_coast.flensburg_to_fehmarn =
            "Flensburg bis Fehmarn: West bis Suedwest 5 bis 6, dabei Boeen von 8 Beaufort."
                .to_string();
        test_output_german_baltic_coast.east_of_fehmarn_to_ruegen =
            "oestlich Fehmarn bis Ruegen: West bis Suedwest 5 bis 6, dabei Boeen von 8 Beaufort."
                .to_string();
        test_output_german_baltic_coast.east_from_ruegen =
            "oestlich Ruegen: West bis Suedwest 5 bis 6, dabei Boeen von 8 Beaufort".to_string();

        test_output_german.north_sea_coast = test_output_german_north_sea_coast;
        test_output_german.baltic_coast = test_output_german_baltic_coast;

        assert_eq!(german, test_output_german);
    }
}
