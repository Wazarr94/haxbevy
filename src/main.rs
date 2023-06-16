use bevy::asset::*;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use parser::stadium::Stadium;
use parser::stadium::StadiumRaw;
use renderer::RendererPlugin;

use jsonc_parser::{parse_to_serde_value, ParseOptions};

mod menu;
mod parser;
mod renderer;

#[derive(Debug, TypeUuid)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
struct StadiumAsset(pub Stadium);

#[derive(Default)]
struct StadiumLoader;

impl AssetLoader for StadiumLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            let stadium_value = parse_to_serde_value(data_str, &ParseOptions::default())?;
            let stadium_raw: StadiumRaw = serde_json::from_value(stadium_value.unwrap())?;
            let stadium = stadium_raw.to_stadium();
            let asset = StadiumAsset(stadium);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json5", "json", "hbs"]
    }
}

#[derive(Debug, Resource)]
struct DataAssets {
    pub stadium: Handle<StadiumAsset>,
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RendererPlugin)
        .add_asset::<StadiumAsset>()
        .init_asset_loader::<StadiumLoader>()
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_path = "stadiums/obstacle-map-winky.json5";
    let stadium: Handle<StadiumAsset> = asset_server.load(map_path);
    commands.insert_resource(DataAssets { stadium });

    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.7, -0.7, -1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
