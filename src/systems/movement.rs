use crate::prelude::*;

#[system(for_each)]
#[read_component(Adventurer)]
pub fn movement(
    entity: &Entity,
    wants_to_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    buffer: &mut CommandBuffer,
) {
    if map.can_enter_tile(&wants_to_move.destination) {
        buffer.add_component(wants_to_move.entity, wants_to_move.destination);
        if let Ok(entry_ref) = ecs.entry_ref(wants_to_move.entity) {
            if entry_ref.get_component::<Adventurer>().is_ok() {
                camera.follow_adventurer(&wants_to_move.destination);
            }
        }
    }
    buffer.remove(*entity);
}
