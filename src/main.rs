use prelude::*;

mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    WALL,
    FLOOR,
}

struct State {
    map: Map,
    adventurer: Adventurer,
}

impl Default for State {
    fn default() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder: MapBuilder = (&mut rng).into();
        let map = map_builder.map;
        let adventurer: Adventurer = map_builder.starting_point.into();

        Self { map, adventurer }
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();

        // respond to input
        self.adventurer.update(context, &self.map);

        // render
        self.map.render(context);
        self.adventurer.render(context);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::default())
}
