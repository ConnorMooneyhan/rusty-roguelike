use crate::prelude::*;

mod player_input;
mod map_render;

/// Builds Legion schedule from systems
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .build()
}