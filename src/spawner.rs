use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, position: &Point) {
    ecs.push((
        Adventurer,
        *position,
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

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, position: &Point) {
    let (hit_points, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        *position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingAdventurer {},
        Health {
            current: hit_points,
            max: hit_points,
        },
        Name(name),
    ));
}

fn goblin() -> (u32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (u32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
