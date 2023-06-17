use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(menu.in_set(OnUpdate(AppState::Menu)))
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

fn setup_menu(mut commands: Commands) {
    println!("setup menu");
    commands.insert_resource(MenuData {
        stadium_info: StadiumInfo::Classic,
    });
}

fn menu(
    // mut commands: Commands,
    mut contexts: EguiContexts,
    // mut next_state: ResMut<NextState<AppState>>,
    mut menu_data: ResMut<MenuData>,
    // asset_server: Res<AssetServer>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", menu_data.stadium_info))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut menu_data.stadium_info,
                        StadiumInfo::Classic,
                        "Classic",
                    );
                    ui.selectable_value(&mut menu_data.stadium_info, StadiumInfo::Big, "Big");
                });
        });
    });
}

fn cleanup_menu(mut commands: Commands) {
    println!("cleanup menu")
}
