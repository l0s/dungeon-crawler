use crate::prelude::*;
use crate::TileType::{FLOOR, WALL};

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let mut batch = DrawBatch::new();
    batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let camera_point = Point::new(x, y);
            let camera_corner = Point::new(camera.left_x, camera.top_y);
            if let Some(tile) = map.get_tile(&camera_point) {
                let glyph = match tile {
                    WALL => to_cp437('#'),
                    FLOOR => to_cp437('.'),
                };
                batch.set(
                    camera_point - camera_corner,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }
    batch.submit(0).expect("Batch rendering error");
}