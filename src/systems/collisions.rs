use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Adventurer)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, buffer: &mut CommandBuffer) {
    let player_position = <&Point>::query()
        .filter(component::<Adventurer>())
        .iter(ecs)
        .last()
        .expect("Player not found");

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, position)| *position == player_position)
        .for_each(|(entity, _)| buffer.remove(*entity));
}