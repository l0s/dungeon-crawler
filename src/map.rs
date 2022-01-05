use crate::prelude::*;
use crate::TileType;
use crate::TileType::{FLOOR, WALL};

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: vec![FLOOR; NUM_TILES],
        }
    }
}

impl Map {
    /// Render the map from the camera's perspective
    pub fn render(&self, context: &mut BTerm, camera: &Camera) {
        context.set_active_console(0); // TODO constant for layers

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let point = Point::new(x, y);
                if let Some(tile) = self.get_tile(&point) {
                    match tile {
                        WALL => context.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                        FLOOR => context.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                    }
                }
            }
        }
    }

    /// Determine if a point is contained in the map
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.y < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: &Point) -> bool {
        self.in_bounds(point) && self.tiles[Self::map_index(point.x, point.y)] == FLOOR
    }

    pub fn get_tile(&self, point: &Point) -> Option<TileType> {
        if self.in_bounds(point) {
            let index = Self::map_index(point.x, point.y);
            return Some(self.tiles[index]);
        }
        None
    }

    pub fn set_tile(&mut self, point: &Point, tile: TileType) {
        if self.in_bounds(point) {
            let index = Self::map_index(point.x, point.y);
            self.tiles[index] = tile;
        }
    }

    fn map_index(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }
}
