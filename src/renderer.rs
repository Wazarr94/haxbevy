use bevy::prelude::*;

use crate::parser::stadium::Stadium;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_stadium);
    }
}

fn draw_stadium(mut commands: Commands, stadium: Res<Stadium>) {
    stadium.bg.draw(&mut commands);

    for (index, disc) in stadium.discs.iter().enumerate() {
        disc.draw(&mut commands, index);
    }

    // get index and the segment from &stadium.segments
    for (index, segment) in stadium.segments.iter().enumerate() {
        segment.draw(&mut commands, &stadium.vertexes, index);
    }
}
