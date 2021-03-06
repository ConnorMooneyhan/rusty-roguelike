use crate::prelude::*;
use template::Templates;

mod template;

/// Spawns level
pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

/// Spawns player
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

/// Spawns Amulet of Paxus
pub fn spawn_amulet_of_paxus(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfPaxus,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet Of Paxus".to_string()),
    ));
}
