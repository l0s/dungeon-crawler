use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_position: &Point,
    #[resource] camera: &Camera,
    positions: &mut Query<(Entity, &Point, &Name)>,
) {
    let map_position = *mouse_position + camera.origin();
    let mut batch = DrawBatch::new();
    batch.target(HUD_LAYER);

    positions
        .iter(ecs)
        .filter(|(_, position, _)| **position == map_position)
        .for_each(|(entity, _, name)| {
            let screen_position = *mouse_position * 4;
            if let Ok(reference) = ecs.entry_ref(*entity) {
                let display = if let Ok(health) = reference.get_component::<Health>() {
                    format!("{}: {} hit points", &name.0.clone(), health.current)
                } else {
                    name.0.clone()
                };
                batch.print(screen_position, &display);
            }
        });
    batch.submit(32_768).expect("Error updating HUD");
}
