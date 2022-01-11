use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    attacker_query: &mut Query<(Entity, &WantsToAttack)>,
    buffer: &mut CommandBuffer,
) {
    let targets = attacker_query
        .iter(ecs)
        .map(|(attacker, intent)| (*attacker, intent.target))
        .collect::<Vec<(Entity, Entity)>>();
    targets.iter().for_each(|(message, target)| {
        if let Ok(mut target_entry) = ecs.entry_mut(*target) {
            if let Ok(mut target_health) = target_entry.get_component_mut::<Health>() {
                target_health.current -= 1;
                if target_health.current < 1 {
                    buffer.remove(*target);
                }
            }
        }
        buffer.remove(*message);
    });
}
