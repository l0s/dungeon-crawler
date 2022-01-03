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

    fn map_index(x: i32, y: i32) -> usize {
        // TODO should probably be on Map
        ((y * SCREEN_WIDTH) + x) as usize
    }
}
