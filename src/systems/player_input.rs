use VirtualKeyCode::{Down, Left, Right, Up};

use TurnState::PlayerTurn;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Adventurer)]
pub fn player_input(
    ecs: &mut SubWorld,
    buffer: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
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
            <(Entity, &Point)>::query()
                .filter(component::<Adventurer>())
                .iter(ecs)
                .for_each(|(entity, position)| {
                    let destination = *position + delta;
                    buffer.push((
                        (), // Legion does not accept single-component insertions
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                });
            *turn_state = PlayerTurn;
        }
    }
}
