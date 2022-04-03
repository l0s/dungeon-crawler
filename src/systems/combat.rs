use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Adventurer)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    attacker_query: &mut Query<(Entity, &WantsToAttack)>,
    buffer: &mut CommandBuffer,
) {
    // TODO no need to collect
    let targets = attacker_query
        .iter(ecs)
        .map(|(attacker, intent)| (*attacker, intent.target))
        .collect::<Vec<(Entity, Entity)>>();
    targets.iter().for_each(|(message, target)| {
        let is_adventurer = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Adventurer>()
            .is_ok();
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && !is_adventurer {
                buffer.remove(*target);
            }
        }
        buffer.remove(*message);
    });
}
