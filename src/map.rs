use bracket_lib::prelude::{BTerm, to_cp437};

use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

/// Calculates index in Map.tiles for a given x and y\
/// Indexed Y-first
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>
}

impl Map {
    /// Constructs new Map instance populated with floor tiles.
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    /// Loops through Map.tiles to render '.' for floor tiles and '#' for wall tiles.
    pub fn render(&self, ctx: &mut BTerm) {
        // Looping y-first is most performant with row-first striding
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(
                            x,
                            y,
                            YELLOW,
                            BLACK,
                            to_cp437('.')
                        );
                    },
                    TileType::Wall => {
                        ctx.set(
                            x,
                            y,
                            GREEN,
                            BLACK,
                            to_cp437('#')
                        );
                    }
                }
            }
        }
    }
}