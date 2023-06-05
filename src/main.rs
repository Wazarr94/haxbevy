use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use parser::stadium::{Stadium, StadiumRaw};
use renderer::RendererPlugin;
use std::{error::Error, fs};

use jsonc_parser::{parse_to_serde_value, ParseOptions};

mod parser;
mod renderer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RendererPlugin)
        .add_startup_system(setup.in_base_set(StartupSet::PreStartup))
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
    let map = read_stadium("assets/stadiums/obstacle-map-winky.json5".to_string()).unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.7, -0.7, -1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(map);
}
