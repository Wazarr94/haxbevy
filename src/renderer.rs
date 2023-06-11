use bevy::prelude::*;

use crate::{DataAssets, StadiumAsset};

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_stadium);
    }
}

// use asset event instead
fn render_stadium(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<StadiumAsset>>,
    stadium_assets: Res<Assets<StadiumAsset>>,
    data_assets: Res<DataAssets>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if *handle != data_assets.stadium {
                continue;
            }

            let stadium = stadium_assets.get(handle).unwrap();
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
    }
}
