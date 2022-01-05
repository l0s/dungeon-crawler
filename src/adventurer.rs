use VirtualKeyCode::{Down, Left, Right, Up};

use crate::prelude::*;

pub struct Adventurer {
    pub position: Point,
}

impl From<Point> for Adventurer {
    fn from(position: Point) -> Self {
        Adventurer { position }
    }
}

impl Default for Adventurer {
    fn default() -> Self {
        Adventurer {
            position: Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
        }
    }
}

impl Adventurer {
    pub fn render(&self, context: &mut BTerm, camera: &Camera) {
        context.set_active_console(1); // TODO constant for layers
        context.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, context: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = context.key {
            let delta = match key {
                Left => Point::new(-1, 0),
                Right => Point::new(1, 0),
                Up => Point::new(0, -1),
                Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let requested_position = self.position + delta;
            if map.can_enter_tile(&requested_position) {
                self.position = requested_position;
                camera.follow_adventurer(self);
            }
        }
    }
}
