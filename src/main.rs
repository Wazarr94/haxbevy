use std::{error::Error, fs};

use jsonc_parser::{parse_to_serde_value, ParseOptions};

use crate::parser::stadium::StadiumRaw;

mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    for stadium_file in fs::read_dir("assets/stadiums")? {
        let stadium_str = fs::read_to_string(stadium_file?.path())?;
        let stadium_value = parse_to_serde_value(&stadium_str, &ParseOptions::default())?.unwrap();
        let stadium_raw: StadiumRaw = serde_json::from_value(stadium_value)?;
        let stadium = stadium_raw.to_stadium();
        println!("Successfully read {}", &stadium.name);
    }
    Ok(())
}
