use crate::prelude::*;

use VirtualKeyCode::{Down, Left, Right, Up};

#[system]
#[write_component(Point)]
#[read_component(Adventurer)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            Left => Point::new(-1, 0),
            Right => Point::new(1, 0),
            Up => Point::new(0, -1),
            Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut adventurers = <&mut Point>::query().filter(component::<Adventurer>());
            adventurers.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(&destination) {
                    *pos = destination;
                    camera.follow_adventurer(&destination);
                }
            });
        }
    }
}
