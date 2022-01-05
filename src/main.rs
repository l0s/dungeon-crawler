use prelude::*;

mod adventurer;
mod camera;
mod map;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::adventurer::*;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    WALL,
    FLOOR,
}

struct State {
    map: Map,
    adventurer: Adventurer,
    camera: Camera,
}

impl Default for State {
    fn default() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder: MapBuilder = (&mut rng).into();
        let map = map_builder.map;
        let adventurer: Adventurer = map_builder.starting_point.into();
        let camera: Camera = (&adventurer).into();

        Self {
            map,
            adventurer,
            camera,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.set_active_console(0); // TODO constant
        context.cls();
        context.set_active_console(1); // TODO constant
        context.cls();

        // respond to input
        self.adventurer.update(context, &self.map, &mut self.camera);

        // render
        self.map.render(context, &self.camera);
        self.adventurer.render(context, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::default())
}
