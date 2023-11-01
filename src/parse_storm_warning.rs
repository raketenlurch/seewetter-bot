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

impl German {}

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

    /*pub fn parse(filename: &str) -> Self {
        let number = Self::get_number(filename).unwrap();
        let title = Self::get_title(number.0).unwrap();
        let east_frisian_coast = Self::get_east_frisian_coast(title.0).unwrap();
        let elbe_estuary = Self::get_elbe_estuary(east_frisian_coast.0).unwrap();
        let sea_area_helgoland = Self::get_sea_area_helgoland(elbe_estuary.0).unwrap();
        let north_frisian_coast = Self::get_north_frisian_coast(sea_area_helgoland.0).unwrap();
        let elbe_from_hamburg_to_cuxhaven =
            Self::get_elbe_from_hamburg_to_cuxhaven(north_frisian_coast.1).unwrap();

        Self {
            number: number.1.to_string(),
            title: title.1.to_string(),
            east_frisian_coast: east_frisian_coast.1.to_string(),
            elbe_estuary: elbe_estuary.1.to_string(),
            sea_area_helgoland: sea_area_helgoland.1.to_string(),
            north_frisian_coast: north_frisian_coast.1.to_string(),
            elbe_from_hamburg_to_cuxhaven: elbe_from_hamburg_to_cuxhaven.1.to_string(),
        }
    }*/

    /*pub fn parse(filename: &str) -> Self {
        let number_with_newline = Self::get_number(filename).unwrap();
        let number = number_with_newline.0.replace("\n", " ");

        let title_with_newline = Self::get_title(&number).unwrap();
        let title = title_with_newline.0.replace("\n", " ");

        let east_frisian_coast_with_newline = Self::get_east_frisian_coast(&title).unwrap();
        let east_frisian_coast = east_frisian_coast_with_newline.0.replace("\n", " ");

        let elbe_estuary_with_newline = Self::get_elbe_estuary(&east_frisian_coast).unwrap();
        let elbe_estuary = elbe_estuary_with_newline.0.replace("\n", " ");

        let sea_area_helgoland_with_newline = Self::get_sea_area_helgoland(&elbe_estuary).unwrap();
        let sea_area_helgoland = sea_area_helgoland_with_newline.0.replace("\n", " ");

        let north_frisian_coast_with_newline =
            Self::get_north_frisian_coast(&sea_area_helgoland).unwrap();
        let north_frisian_coast = north_frisian_coast_with_newline.0.replace("\n", " ");

        let elbe_from_hamburg_to_cuxhaven_with_newline =
            Self::get_elbe_from_hamburg_to_cuxhaven(&north_frisian_coast).unwrap();
        let elbe_from_hamburg_to_cuxhaven = elbe_from_hamburg_to_cuxhaven_with_newline
            .0
            .replace("\n", " ");

        Self {
            number,
            title,
            east_frisian_coast,
            elbe_estuary,
            sea_area_helgoland,
            north_frisian_coast,
            elbe_from_hamburg_to_cuxhaven,
        }
    }*/

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

#[allow(unused)]
impl HighWindWarningGermanBalticCoast {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            number: String::new(),
            title: String::new(),
            flensburg_to_fehmarn: String::new(),
            east_of_fehmarn_to_ruegen: String::new(),
            east_from_ruegen: String::new(),
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
        dbg!(&high_wind_warning_north_sea_coast);

        let mut test_output = HighWindWarningGermanNorthSeaCoast::new();
        test_output.number = "NR. 437".to_string();
        test_output.title = "Amtliche STARKWIND-Warnung des Seewetterdienstes Hamburg fuer die deutsche Nordseekueste herausgegeben am Mittwoch, den 04.10.2023 um 08:30 Uhr GZ"
            .to_string();
        test_output.east_frisian_coast =
            "Ostfriesische Kueste:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output.elbe_estuary =
            "Elbmuendung:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort.".to_string();
        test_output.sea_area_helgoland =
            "Seegebiet Helgoland:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output.north_frisian_coast =
            "Nordfriesische Kueste:  Suedwest bis West 6 bis 7, dabei Boeen von 9 Beaufort."
                .to_string();
        test_output.elbe_from_hamburg_to_cuxhaven = "Elbe von Hamburg bis Cuxhaven:  Westteil Suedwest bis West 5 bis 6, dabei Boeen von 8 Beaufort"
            .to_string();

        assert_eq!(high_wind_warning_north_sea_coast, test_output);
    }
}
