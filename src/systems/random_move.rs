use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    // FIXME monsters are running into each other
    let mut rng = RandomNumberGenerator::new();
    <(&mut Point, &MovingRandomly)>::query()
        .iter_mut(ecs)
        .for_each(|(position, _)| {
            let delta = Point::new(rng.range(-1, 2), rng.range(-1, 2));
            let destination = *position + delta;
            if map.can_enter_tile(&destination) {
                *position = destination;
            }
        });
}
