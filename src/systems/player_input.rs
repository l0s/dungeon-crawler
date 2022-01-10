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
            if let Some((adventurer, destination)) =
                <(Entity, &Point)>::query() // TODO pull this into the argument list
                    .filter(component::<Adventurer>())
                    .iter(ecs)
                    .find_map(|(entity, position)| Some((*entity, *position + delta)))
            {
                let mut hit_something = false;
                <(Entity, &Point)>::query()
                    .filter(component::<Enemy>())
                    .iter(ecs)
                    .filter(|(_, position)| **position == destination)
                    .for_each(|(target, _)| {
                        hit_something |= true;
                        buffer.push((
                            (), // Legion does not accept single-component insertions
                            WantsToAttack {
                                attacker: adventurer,
                                target: *target,
                            },
                        ));
                    });
                if !hit_something {
                    buffer.push((
                        (), // Legion does not accept single-component insertions
                        WantsToMove {
                            entity: adventurer,
                            destination,
                        },
                    ));
                }
            }
            *turn_state = PlayerTurn;
        }
    }
}
