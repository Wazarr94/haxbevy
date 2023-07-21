use crate::{
    menu::{DataAssets, StadiumAsset},
    parser::utils::Position,
};
use bevy::{prelude::*, render::camera::ScalingMode};

use crate::AppState;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (spawn_stadium,))
            .add_systems(Update, draw_discs.run_if(in_state(AppState::InGame)));
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

fn draw_discs(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x as f32;
        transform.translation.y = position.0.y as f32;
    }
}
