use prelude::*;

use crate::prelude::TurnState::{AwaitingInput, GameOver, MonsterTurn, PlayerTurn};

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    // Dimensions
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    // Layers
    pub const MAP_LAYER: usize = 0;
    pub const CHARACTER_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    WALL,
    FLOOR,
}

struct State {
    /// entity component system
    ecs: World,
    resources: Resources,

    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl Default for State {
    fn default() -> Self {
        let mut ecs = World::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder: MapBuilder = MapBuilder::from(&mut rng);

        // create entities
        spawn_player(&mut ecs, &map_builder.starting_point);
        spawn_monsters(&mut ecs, &map_builder, &mut rng);

        Self {
            ecs,
            resources: create_resources(map_builder, rng),
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

fn spawn_monsters(ecs: &mut World, map_builder: &MapBuilder, rng: &mut RandomNumberGenerator) {
    map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|room| room.center())
        .for_each(|position| spawn_monster(ecs, rng, &position));
}

fn create_resources(map_builder: MapBuilder, rng: RandomNumberGenerator) -> Resources {
    let mut resources = Resources::default();
    resources.insert(map_builder.map);
    resources.insert(Camera::from(&map_builder.starting_point));
    resources.insert(AwaitingInput);
    resources.insert(rng);
    resources
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.set_active_console(MAP_LAYER);
        context.cls();
        context.set_active_console(CHARACTER_LAYER);
        context.cls();
        context.set_active_console(HUD_LAYER);
        context.cls();

        // input
        self.resources.insert(context.key);
        context.set_active_console(MAP_LAYER);
        self.resources
            .insert(Point::from_tuple(context.mouse_pos()));

        // mechanics
        let turn_state = *self
            .resources
            .get::<TurnState>()
            .expect("Missing turn state");
        if turn_state == GameOver {
            self.game_over(context);
        } else {
            let systems_scheduler = match turn_state {
                AwaitingInput => &mut self.input_systems,
                PlayerTurn => &mut self.player_systems,
                MonsterTurn => &mut self.monster_systems,
                GameOver => panic!("Unexpected state"),
            };
            systems_scheduler.execute(&mut self.ecs, &mut self.resources);
        }

        // render
        render_draw_buffer(context).expect("Render error");
    }
}

impl State {
    fn game_over(&mut self, context: &mut BTerm) {
        context.set_active_console(HUD_LAYER);
        context.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        context.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        context.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yara remains unclaimed, and your home town is not saved.",
        );
        context.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        context.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = context.key {
            self.ecs = World::default();
            let mut rng = RandomNumberGenerator::new();
            let map_builder = MapBuilder::from(&mut rng);

            // create entities
            spawn_player(&mut self.ecs, &map_builder.starting_point);
            spawn_monsters(&mut self.ecs, &map_builder, &mut rng);

            self.resources = create_resources(map_builder, rng);
            self.resources.insert(TurnState::AwaitingInput);
        }
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
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::default())
}
