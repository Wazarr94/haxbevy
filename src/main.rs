use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use debug::DebugPlugin;
use menu::MenuPlugin;
use physics::PhysicsPlugin;
use renderer::RendererPlugin;

mod debug;
mod menu;
mod parser;
mod physics;
mod renderer;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_state::<AppState>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EguiPlugin,
            ShapePlugin,
            DebugPlugin,
            MenuPlugin,
            RendererPlugin,
            PhysicsPlugin,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}
