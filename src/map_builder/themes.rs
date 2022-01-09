use crate::prelude::*;

/// Dungeon Theme
pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'), // Dungeon Floor
            TileType::Wall => to_cp437('#'),  // Dungeon Wall
            TileType::Exit => to_cp437('>'),  // Stairs
        }
    }
}

/// Forest Theme
pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'), // Forest Floor
            TileType::Wall => to_cp437('"'),  // Tree
            TileType::Exit => to_cp437('>'),  // Stairs
        }
    }
}
