use crate::prelude::*;

/// Spawns player
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
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
    ));
}

/// Spawns Amulet of Paxus
pub fn spawn_amulet_of_paxus(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Item,
            AmuletOfPaxus,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|')
            },
            Name("Amulet Of Paxus".to_string())
        )
    );
}

/// Returns tuple containing hitpoint count, name, and FontCharType for a goblin character
fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

/// Returns tuple containing hitpoint count, name, and FontCharType for an orc character
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
