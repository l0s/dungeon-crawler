use VirtualKeyCode::{Down, Left, Right, Up};

use crate::prelude::*;

pub struct Adventurer {
    position: Point,
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
    pub fn render(&self, context: &mut BTerm) {
        context.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, context: &mut BTerm, map: &Map) {
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
            }
        }
    }
}
