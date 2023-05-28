use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use parser::stadium::{Stadium, StadiumRaw};
use std::{error::Error, fs};

use jsonc_parser::{parse_to_serde_value, ParseOptions};

mod parser;

fn read_stadium(file_name: String) -> Result<Stadium, Box<dyn Error>> {
    let stadium_str = fs::read_to_string(file_name)?;
    let stadium_value = parse_to_serde_value(&stadium_str, &ParseOptions::default())?.unwrap();
    let stadium_raw: StadiumRaw = serde_json::from_value(stadium_value)?;
    let stadium = stadium_raw.to_stadium();
    println!("Segments: {:#?}", stadium.segments);
    Ok(stadium)
}

#[allow(dead_code)]
fn read_stadiums() -> Result<(), Box<dyn Error>> {
    for stadium_file in fs::read_dir("assets/stadiums")? {
        let file_name = stadium_file?.path().display().to_string();
        let stadium = read_stadium(file_name)?;
        println!("Successfully read {}", &stadium.name);
    }
    Ok(())
}

fn setup(mut commands: Commands) {
    let classic_stadium = read_stadium("assets/stadiums/classic.json5".to_string()).unwrap();

    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(classic_stadium);
}
fn draw_stadium(mut commands: Commands, stadium: Res<Stadium>) {
    stadium.bg.draw(&mut commands);

    for disc in &stadium.discs {
        disc.draw(&mut commands);
    }

    for segment in &stadium.segments {
        segment.draw(&mut commands, &stadium.vertexes);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(draw_stadium)
        .run();
}
