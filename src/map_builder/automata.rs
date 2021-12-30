use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::create_empty();
        
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            *t = if roll > 55 {
                TileType::Floor
            } else {
                TileType::Wall
            };
        })
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if (ix != 0 || iy != 0) && map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT-1 {
            for x in 1..SCREEN_WIDTH-1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                new_tiles[idx] = match neighbors > 4 || neighbors == 0 {
                    true => TileType::Wall,
                    false => TileType::Floor,
                };
            }
        }
        map.tiles = new_tiles;
    }
}