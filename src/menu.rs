use bevy::{asset::*, prelude::*, reflect::TypeUuid};
use bevy_egui::{egui, EguiContexts};
use jsonc_parser::{parse_to_serde_value, ParseOptions};

use crate::{
    parser::stadium::{Stadium, StadiumRaw},
    AppState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<StadiumAsset>()
            .init_asset_loader::<StadiumLoader>()
            .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_systems((menu, load_to_ingame).in_set(OnUpdate(AppState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

#[derive(Resource, Default)]
struct MenuData {
    stadium_info: StadiumInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum StadiumInfo {
    #[default]
    Classic,
    Big,
}

impl StadiumInfo {
    fn get_path(&self) -> &str {
        match self {
            StadiumInfo::Classic => "stadiums/classic.json5",
            StadiumInfo::Big => "stadiums/big.json5",
        }
    }
}

#[derive(Resource, Default)]
struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Resource)]
pub struct DataAssets {
    pub stadium: Handle<StadiumAsset>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
pub struct StadiumAsset(pub Stadium);

#[derive(Default)]
pub struct StadiumLoader;

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

fn setup_menu(mut commands: Commands) {
    commands.insert_resource(AssetsLoading::default());
    commands.insert_resource(MenuData {
        stadium_info: StadiumInfo::Classic,
    });
}

fn menu(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut menu_data: ResMut<MenuData>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("Stadiums");

        egui::ComboBox::from_id_source("stadium")
            .selected_text(format!("{:?}", menu_data.stadium_info))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut menu_data.stadium_info, StadiumInfo::Classic, "Classic");
                ui.selectable_value(&mut menu_data.stadium_info, StadiumInfo::Big, "Big");
            });

        if ui.button("Load").clicked() {
            let stadium = asset_server.load(menu_data.stadium_info.get_path());
            loading.0.push(stadium.clone_untyped());
            commands.insert_resource(DataAssets { stadium });
        }
    });
}

fn load_to_ingame(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if loading.0.is_empty() {
        return;
    }

    match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            panic!("Failed to load assets");
        }
        LoadState::Loaded => {
            println!("Assets loaded");
            commands.remove_resource::<AssetsLoading>();
            next_state.set(AppState::InGame);
        }
        _ => {}
    }
}

fn cleanup_menu() {
    println!("cleanup menu")
}
