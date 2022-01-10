use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, buffer: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query(); // TODO pull this into the argument list
    let targets = attackers
        .iter(ecs)
        .map(|(attacker, intent)| (*attacker, intent.target))
        .collect::<Vec<(Entity, Entity)>>();
    targets.iter().for_each(|(message, target)| {
        if let Ok(mut target_entry) = ecs.entry_mut(*target) {
            if let Ok(mut target_health) = target_entry.get_component_mut::<Health>() {
                eprintln!("Health before attack: {}", target_health.current);
                target_health.current -= 1;
                if target_health.current < 1 {
                    buffer.remove(*target);
                }
                eprintln!("Health after attack: {}", target_health.current);
            }
        }
        buffer.remove(*message);
    });
}
