use crate::prelude::*;
use crate::TileType;
use crate::TileType::FLOOR;

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
            if index < NUM_TILES {
                return Some(self.tiles[index]);
            }
        }
        None
    }

    pub fn set_tile(&mut self, point: &Point, tile: TileType) {
        if self.in_bounds(point) {
            let index = Self::map_index(point.x, point.y);
            self.tiles[index] = tile;
        }
    }

    pub fn map_index(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    /// Determine if a tile is a valid exit
    ///
    /// Parameters:
    /// - `source` - the tile from which a character is moving
    /// - `delta` - the movement vector
    /// Returns:
    /// - `Some(usize)` - the tile index of the destination
    /// - `None` - if moving to that tile is not valid
    fn valid_exit(&self, source: Point, delta: Point) -> Option<usize> {
        let destination = source + delta;
        if self.can_enter_tile(&destination) {
            let index = self.point2d_to_index(destination);
            Some(index)
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, source_index: usize) -> SmallVec<[(usize, f32); 10]> {
        let source_location = self.index_to_point2d(source_index);
        vec![
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ]
        .iter()
        .filter_map(|direction| self.valid_exit(source_location, *direction))
        .map(|destination_index| (destination_index, 1.0))
        .collect::<SmallVec<[(usize, f32); 10]>>()
    }

    fn get_pathing_distance(&self, x: usize, y: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(x), self.index_to_point2d(y))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(&point)
    }
}
