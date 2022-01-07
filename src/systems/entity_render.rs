use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(1); // TODO constant for layers
    let camera_corner = Point::new(camera.left_x, camera.top_y); // TODO add method to Camera e.g. Camera::origin

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(position, render)| {
            batch.set(*position - camera_corner, render.color, render.glyph);
        });
    batch.submit(8192).expect("Error rendering entities");
}