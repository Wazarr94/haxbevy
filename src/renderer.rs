use crate::menu::{DataAssets, StadiumAsset};
use bevy::{prelude::*, render::camera::ScalingMode};

use crate::AppState;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_stadium,).in_schedule(OnEnter(AppState::InGame)));
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
