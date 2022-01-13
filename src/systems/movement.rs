use std::collections::HashSet;

use crate::prelude::*;

#[system(for_each)]
#[read_component(Adventurer)]
#[read_component(Point)]
pub fn movement(
    movement_intent: &Entity,
    wants_to_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    buffer: &mut CommandBuffer,
) {
    let occupied_tiles = <(Entity, &Point)>::query()
        .iter(ecs)
        .map(|(_entity, position)| position)
        .collect::<HashSet<&Point>>();
    if map.can_enter_tile(&wants_to_move.destination)
        && !occupied_tiles.contains(&wants_to_move.destination)
    {
        buffer.add_component(wants_to_move.entity, wants_to_move.destination);
        if let Ok(entry_ref) = ecs.entry_ref(wants_to_move.entity) {
            if entry_ref.get_component::<Adventurer>().is_ok() {
                camera.follow_adventurer(&wants_to_move.destination);
            }
        }
    }
    buffer.remove(*movement_intent);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_two_monsters_do_not_enter_same_space() {
        // given
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut buffer = CommandBuffer::new(&mut world);
        let mut camera = Camera::from(&Point::new(0, 0));
        let mut map = Map::default();

        let orc = (
            Point::new(1, 1),
            Health { current: 0, max: 0 },
            Enemy {},
            MovingRandomly {},
        );
        let goblin = (
            Point::new(3, 3),
            Health { current: 0, max: 0 },
            Enemy {},
            MovingRandomly {},
        );

        let orc_entity = world.push(orc);
        let goblin_entity = world.push(goblin);

        // the orc and goblin want to move into the same space
        let orc_intent = WantsToMove {
            entity: orc_entity.clone(),
            destination: Point::new(2, 2),
        };
        let goblin_intent = WantsToMove {
            entity: goblin_entity.clone(),
            destination: Point::new(2, 2),
        };

        // when
        movement(
            &world.push(((), orc_intent)),
            &orc_intent,
            &mut map,
            &mut camera,
            &mut SubWorld::from(&mut world),
            &mut buffer,
        );
        assert_eq!(buffer.len(), 2); // move orc, delete message
        buffer.flush(&mut world, &mut resources);
        assert!(buffer.is_empty());
        let orc_entry = world.entry(orc_entity).expect("Orc entry not found");
        let orc_position = *orc_entry
            .get_component::<Point>()
            .expect("Orc position not found");
        assert_eq!(orc_position, Point::new(2, 2), "Orc should have moved");

        movement(
            &world.push(((), goblin_intent)),
            &goblin_intent,
            &mut map,
            &mut camera,
            &mut SubWorld::from(&mut world),
            &mut buffer,
        );
        assert_eq!(buffer.len(), 1); // delete message
        buffer.flush(&mut world, &mut resources);

        let goblin_entry = world.entry(goblin_entity).expect("Goblin entry not found");
        let goblin_position = *goblin_entry
            .get_component::<Point>()
            .expect("Goblin position not found");
        assert_eq!(
            goblin_position,
            Point::new(3, 3),
            "Goblin should not have moved"
        );
    }
}
