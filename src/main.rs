use prelude::*;

mod map;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::map::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    WALL,
    FLOOR,
}

#[derive(Default)]
struct State {
    map: Map,
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();
        self.map.render(context);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        ?.with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::default())
}
