use crate::prelude::*;

#[system(for_each)]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Adventurer)]
pub fn random_move(
    entity: &Entity,
    position: &Point,
    _component: &MovingRandomly,
    ecs: &mut SubWorld,
    buffer: &mut CommandBuffer,
) {
    // FIXME monsters are running into each other
    let mut rng = RandomNumberGenerator::new();

    let mut targets = <(Entity, &Point, &Health)>::query();

    let delta = Point::new(rng.range(-1, 2), rng.range(-1, 2));
    let destination = *position + delta;

    let mut destination_occupied = false;
    targets
        .iter(ecs)
        .filter(|(_, target_position, _)| **target_position == destination)
        .for_each(|(target, _, _)| {
            if let Ok(target_reference) = ecs.entry_ref(*target) {
                if target_reference.get_component::<Adventurer>().is_ok() {
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
}
