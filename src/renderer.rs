use crate::menu::{DataAssets, StadiumAsset};
use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

use crate::AppState;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_stadium,).in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_canvas_size);
    }
}

fn spawn_stadium(
    mut commands: Commands,
    stadium_assets: Res<Assets<StadiumAsset>>,
    data_assets: Res<DataAssets>,
) {
    let stadium = stadium_assets.get(&data_assets.stadium).unwrap();
    let st = &stadium.0;

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: st.width as f32,
            scaling_mode: ScalingMode::FixedHorizontal(2.0),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(1.0, -1.0, -1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    st.spawn(&mut commands);
}

#[cfg(target_family = "wasm")]
fn update_canvas_size(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    (|| {
        let mut window = window.get_single_mut().ok()?;
        let browser_window = web_sys::window()?;
        let width = browser_window.inner_width().ok()?.as_f64()?;
        let height = browser_window.inner_height().ok()?.as_f64()?;
        window.resolution.set(width as f32, height as f32);
        Some(())
    })();
}
