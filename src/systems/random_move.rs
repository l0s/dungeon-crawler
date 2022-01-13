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
    #[resource] rng: &mut RandomNumberGenerator,
    ecs: &mut SubWorld,
    buffer: &mut CommandBuffer,
) {
    let mut targets = <(Entity, &Point, &Health)>::query();

    // TODO only try to move to open spaces
    let delta = Point::new(rng.range(-1, 2), rng.range(-1, 2));
    let destination = *position + delta;

    let mut destination_occupied = false;
    targets
        .iter(ecs)
        .filter(|(_target, target_position, _health)| **target_position == destination)
        .for_each(|(target, _target_position, _health)| {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_monsters_does_not_enter_occupied_space() {
        // given
        let mut world = World::default();

        let mut rng = RandomNumberGenerator::seeded(0); // re-seed this later
        let orc = (
            Point::new(1, 1),
            Health { current: 0, max: 0 },
            MovingRandomly {},
        );
        let _orc_entity = world.push(orc);
        let delta = Point::new(rng.range(-1, 2), rng.range(-1, 2));
        let goblin = (
            orc.0 - delta,
            Health { current: 0, max: 0 },
            MovingRandomly {},
        );
        let goblin_entity = world.push(goblin);

        let mut rng = RandomNumberGenerator::seeded(0); // re-seed so the goblin tries to displace the orc
        let mut buffer = CommandBuffer::new(&world);
        let mut ecs = SubWorld::from(&mut world);

        // when
        random_move(
            &goblin_entity,
            &goblin.0,
            &goblin.2,
            &mut rng,
            &mut ecs,
            &mut buffer,
        );

        // then
        assert!(buffer.is_empty());
    }
}
