use crate::prelude::*;

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
    ));
}

/// Spawns one of Ettin, Ogre, Orc, or Goblin (random)
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6),
    ));
}

/// Spawns items and monsters
pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_dungeon_map(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    }
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

/// Spawns Healing Potion
pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

/// Spawns Dungeon Map
pub fn spawn_dungeon_map(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap,
    ));
}

/// Returns tuple containing hitpoint count, name, and FontCharType for a goblin character
fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

/// Returns tuple containing hitpoint count, name, and FontCharType for an orc character
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
