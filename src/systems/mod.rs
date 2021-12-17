use crate::prelude::*;

mod player_input;

/// Builds Legion schedule from systems
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}