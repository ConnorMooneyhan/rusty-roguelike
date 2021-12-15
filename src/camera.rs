use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    /// Constructs new `Camera` centered on player
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position - DISPLAY_WIDTH / 2,
            right_x: player_position + DISPLAY_WIDTH / 2,
            top_y: player_position - DISPLAY_HEIGHT / 2,
            bottom_y: player_position + DISPLAY_HEIGHT / 2,
        }
    }

    /// Centers camera on player after player moves
    pub fn on_player_move(&mut self, player_position: Point) {
            self.left_x = player_position - DISPLAY_WIDTH / 2;
            self.right_x = player_position + DISPLAY_WIDTH / 2;
            self.top_y = player_position - DISPLAY_HEIGHT / 2;
            self.bottom_y = player_position + DISPLAY_HEIGHT / 2;
    }
}