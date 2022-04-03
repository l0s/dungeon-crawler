use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingAdventurer)]
#[read_component(Health)]
#[read_component(Adventurer)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, buffer: &mut CommandBuffer) {
    let mut chasers = <(Entity, &Point, &ChasingAdventurer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut adventurer = <(&Point, &Adventurer)>::query();

    // TODO allow for multiple search targets
    let adventurer_location = adventurer.iter(ecs).next().unwrap().0;
    let adventurer_index = map.point2d_to_index(*adventurer_location);

    let search_targets = vec![adventurer_index];
    let flow_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    // TODO emit commands
    chasers.iter(ecs).for_each(|(entity, position, _health)| {
        let index = Map::map_index(position.x, position.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&flow_map, index, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*position, *adventurer_location);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *adventurer_location
            };

            // TODO emit commands
            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_entity, target_position, _health)| **target_position == destination)
                .for_each(|(target, _target_position, _health)| {
                    if ecs
                        .entry_ref(*target)
                        .unwrap()
                        .get_component::<Adventurer>()
                        .is_ok()
                    {
                        buffer.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                target: *target,
                            },
                        ));
                        attacked = true;
                    }
                });
            if !attacked {
                buffer.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
