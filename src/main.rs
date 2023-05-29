use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use parser::stadium::{Stadium, StadiumRaw};
use std::{error::Error, fs};

use jsonc_parser::{parse_to_serde_value, ParseOptions};

mod parser;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_startup_system(draw_stadium.in_base_set(StartupSet::PostStartup))
        .run();
}

fn read_stadium(file_name: String) -> Result<Stadium, Box<dyn Error>> {
    let stadium_str = fs::read_to_string(file_name)?;
    let stadium_value = parse_to_serde_value(&stadium_str, &ParseOptions::default())?.unwrap();
    let stadium_raw: StadiumRaw = serde_json::from_value(stadium_value)?;
    let stadium = stadium_raw.to_stadium();
    Ok(stadium)
}

fn setup(mut commands: Commands) {
    let map = read_stadium("assets/stadiums/classic.json5".to_string()).unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.7, -0.7, -1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(map);
}

fn draw_stadium(mut commands: Commands, stadium: Res<Stadium>) {
    stadium.bg.draw(&mut commands);

    for (index, disc) in stadium.discs.iter().enumerate() {
        disc.draw(&mut commands, index);
    }

    // get index and the segment from &stadium.segments
    for (index, segment) in stadium.segments.iter().enumerate() {
        segment.draw(&mut commands, &stadium.vertexes, index);
    }
}
