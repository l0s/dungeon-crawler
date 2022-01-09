use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, buffer: &mut CommandBuffer) {
    // FIXME monsters are running into each other
    let mut rng = RandomNumberGenerator::new();
    <(Entity, &Point, &MovingRandomly)>::query()
        .iter(ecs)
        .for_each(|(entity, position, _)| {
            let delta = Point::new(rng.range(-1, 2), rng.range(-1, 2));
            let destination = *position + delta;
            buffer.push((
                (), // Legion does not accept single-component insertions
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
}
