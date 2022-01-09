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
            current: 20,
            max: 20,
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
        MovingRandomly {},
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
