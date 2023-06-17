use crate::menu::{DataAssets, StadiumAsset};
use bevy::prelude::*;

use crate::AppState;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_stadium.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn render_stadium(
    mut commands: Commands,
    stadium_assets: Res<Assets<StadiumAsset>>,
    data_assets: Res<DataAssets>,
) {
    let stadium = stadium_assets.get(&data_assets.stadium).unwrap();
    let st = &stadium.0;

    st.bg.draw(&mut commands);

    for (index, disc) in st.discs.iter().enumerate() {
        disc.draw(&mut commands, index);
    }

    // get index and the segment from &stadium.segments
    for (index, segment) in st.segments.iter().enumerate() {
        segment.draw(&mut commands, &st.vertexes, index);
    }
}
