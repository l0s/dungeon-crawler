use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(CHARACTER_LAYER);
    let camera_corner = camera.origin();

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(position, render)| {
            batch.set(*position - camera_corner, render.color, render.glyph);
        });
    batch.submit(8192).expect("Error rendering entities");
}
