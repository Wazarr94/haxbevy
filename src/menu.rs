use bevy::{asset::*, prelude::*, reflect::TypeUuid};
use bevy_egui::{egui, EguiContexts, EguiSettings};
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
    base_stadium_info: BaseStadiumInfo,
    custom_stadium_info: CustomStadiumInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum BaseStadiumInfo {
    #[default]
    Classic,
    Easy,
    Small,
    Big,
    Rounded,
    Hockey,
    BigHockey,
    BigEasy,
    BigRounded,
    Huge,
}

impl BaseStadiumInfo {
    fn get_name(&self) -> &str {
        match self {
            BaseStadiumInfo::Classic => "Classic",
            BaseStadiumInfo::Easy => "Easy",
            BaseStadiumInfo::Small => "Small",
            BaseStadiumInfo::Big => "Big",
            BaseStadiumInfo::Rounded => "Rounded",
            BaseStadiumInfo::Hockey => "Hockey",
            BaseStadiumInfo::BigHockey => "Big Hockey",
            BaseStadiumInfo::BigEasy => "Big Easy",
            BaseStadiumInfo::BigRounded => "Big Rounded",
            BaseStadiumInfo::Huge => "Huge",
        }
    }

    fn get_path(&self) -> &str {
        match self {
            BaseStadiumInfo::Classic => "stadiums/base/classic.json5",
            BaseStadiumInfo::Easy => "stadiums/base/easy.json5",
            BaseStadiumInfo::Small => "stadiums/base/small.json5",
            BaseStadiumInfo::Big => "stadiums/base/big.json5",
            BaseStadiumInfo::Rounded => "stadiums/base/rounded.json5",
            BaseStadiumInfo::Hockey => "stadiums/base/hockey.json5",
            BaseStadiumInfo::BigHockey => "stadiums/base/big_hockey.json5",
            BaseStadiumInfo::BigEasy => "stadiums/base/big_easy.json5",
            BaseStadiumInfo::BigRounded => "stadiums/base/big_rounded.json5",
            BaseStadiumInfo::Huge => "stadiums/base/huge.json5",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum CustomStadiumInfo {
    #[default]
    FutsalClassic,
    FutsalBig,
    PenaltySoccer,
    ObstacleWinky,
    FightingSingle,
}

impl CustomStadiumInfo {
    fn get_name(&self) -> &str {
        match self {
            CustomStadiumInfo::FutsalClassic => "Futsal Classic",
            CustomStadiumInfo::FutsalBig => "Futsal Big",
            CustomStadiumInfo::PenaltySoccer => "Penalty Soccer",
            CustomStadiumInfo::ObstacleWinky => "Obstacle Winky",
            CustomStadiumInfo::FightingSingle => "Fighting Single",
        }
    }

    fn get_path(&self) -> &str {
        match self {
            CustomStadiumInfo::FutsalClassic => "stadiums/custom/futsal-classic.json5",
            CustomStadiumInfo::FutsalBig => "stadiums/custom/futsal-big.json5",
            CustomStadiumInfo::PenaltySoccer => "stadiums/custom/penalty-soccer.json5",
            CustomStadiumInfo::ObstacleWinky => "stadiums/custom/obstacle-map-winky.json5",
            CustomStadiumInfo::FightingSingle => "stadiums/custom/fighting-single.json5",
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

fn setup_menu(mut commands: Commands, mut egui_settings: ResMut<EguiSettings>) {
    egui_settings.scale_factor = 2.0;
    commands.insert_resource(AssetsLoading::default());
    commands.insert_resource(MenuData {
        base_stadium_info: BaseStadiumInfo::default(),
        custom_stadium_info: CustomStadiumInfo::default(),
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

        // Base stadiums
        ui.horizontal(|ui| {
            ui.label("Base stadiums");
            egui::ComboBox::from_id_source("base_stadium")
                .selected_text(format!("{:?}", menu_data.base_stadium_info))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Classic,
                        BaseStadiumInfo::Classic.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Easy,
                        BaseStadiumInfo::Easy.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Small,
                        BaseStadiumInfo::Small.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Big,
                        BaseStadiumInfo::Big.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Rounded,
                        BaseStadiumInfo::Rounded.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Hockey,
                        BaseStadiumInfo::Hockey.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::BigHockey,
                        BaseStadiumInfo::BigHockey.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::BigEasy,
                        BaseStadiumInfo::BigEasy.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::BigRounded,
                        BaseStadiumInfo::BigRounded.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.base_stadium_info,
                        BaseStadiumInfo::Huge,
                        BaseStadiumInfo::Huge.get_name(),
                    );
                });
            ui.add_space(8.0);
            if ui.button("Load base").clicked() {
                let stadium = asset_server.load(menu_data.base_stadium_info.get_path());
                loading.0.push(stadium.clone_untyped());
                commands.insert_resource(DataAssets { stadium });
            }
        });

        ui.add_space(4.0);

        // Custom stadiums
        ui.horizontal(|ui| {
            ui.label("Custom stadiums");
            egui::ComboBox::from_id_source("custom_stadium")
                .selected_text(format!("{:?}", menu_data.custom_stadium_info))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut menu_data.custom_stadium_info,
                        CustomStadiumInfo::FutsalClassic,
                        CustomStadiumInfo::FutsalClassic.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.custom_stadium_info,
                        CustomStadiumInfo::FutsalBig,
                        CustomStadiumInfo::FutsalBig.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.custom_stadium_info,
                        CustomStadiumInfo::PenaltySoccer,
                        CustomStadiumInfo::PenaltySoccer.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.custom_stadium_info,
                        CustomStadiumInfo::ObstacleWinky,
                        CustomStadiumInfo::ObstacleWinky.get_name(),
                    );
                    ui.selectable_value(
                        &mut menu_data.custom_stadium_info,
                        CustomStadiumInfo::FightingSingle,
                        CustomStadiumInfo::FightingSingle.get_name(),
                    );
                });
            ui.add_space(8.0);
            if ui.button("Load custom").clicked() {
                let stadium = asset_server.load(menu_data.custom_stadium_info.get_path());
                loading.0.push(stadium.clone_untyped());
                commands.insert_resource(DataAssets { stadium });
            }
        });
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
