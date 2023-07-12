use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

const DEBUG_MODE: bool = false;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !DEBUG_MODE {
            return;
        }

        app.add_plugins((FrameTimeDiagnosticsPlugin, WorldInspectorPlugin::new()))
            .add_systems(Update, show_fps);
    }
}

fn show_fps(diagnostics: Res<DiagnosticsStore>, mut contexts: EguiContexts) {
    let mut fps_value = 0.0;
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            fps_value = value;
        }
    }

    egui::Window::new("FPS")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!("{:.0}", fps_value));
        });
}
