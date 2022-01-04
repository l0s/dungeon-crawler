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
    pub fn render(&self, context: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = Map::map_index(x, y);
                if let Some(tile_type) = self.tiles.get(index) {
                    match tile_type {
                        WALL => context.set(x, y, YELLOW, BLACK, to_cp437('.')),
                        FLOOR => context.set(x, y, GREEN, BLACK, to_cp437('#')),
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
