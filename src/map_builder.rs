use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    /// Constructs new `MapBuilder` for given `Map` and `player_start` with empty `rooms` vector    
    fn new(map: Map, player_start: Point) -> Self {
        Self {
            map,
            rooms: Vec::new(),
            player_start
        }
    }

    /// Fills entire dungeon with tiles of type `tile`
    fn fill(&mut self, tile: TileType) {
        self.map.tiles
            .iter_mut()
            .for_each(|t| *t = tile);
    }

    /// Carves rooms out of wall (or whatever else was on the map)
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }
}