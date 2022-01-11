use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Adventurer)]
pub fn random_move(ecs: &mut SubWorld, buffer: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();

    let mut targets = <(Entity, &Point, &Health)>::query();

    <(Entity, &Point, &MovingRandomly)>::query() // TODO pull this into the argument list
        .iter(ecs)
        .for_each(|(entity, position, _)| {
            let delta = Point::new(rng.range(-1, 2), rng.range(-1, 2));
            let destination = *position + delta;

            let mut destination_occupied = false;
            targets
                .iter(ecs)
                .filter(|(_, target_position, _)| **target_position == destination)
                .for_each(|(target, _, _)| {
                    if let Ok(target_reference) = ecs.entry_ref(*target) {
                        if target_reference.get_component::<Adventurer>().is_ok() {
                            // TODO can we pull this into the filter?
                            buffer.push((
                                (), // Legion does not accept single-component insertions
                                WantsToAttack {
                                    attacker: *entity,
                                    target: *target,
                                },
                            ));
                        }
                        destination_occupied |= true;
                    }
                });
            if !destination_occupied {
                buffer.push((
                    (), // Legion does not accept single-component insertions
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        });
}
