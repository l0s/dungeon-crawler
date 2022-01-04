use crate::prelude::*;
use crate::TileType;
use crate::TileType::{FLOOR, WALL};

const NUM_ROOMS: usize = 16;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub starting_point: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let candidate = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let overlaps = self.rooms.iter().any(|room| room.intersect(&candidate));
            if !overlaps {
                candidate.for_each(|point| self.map.set_tile(&point, FLOOR));
                self.rooms.push(candidate);
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let previous = rooms[i - 1].center();
            let current = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(previous.x, current.x, previous.y);
                self.apply_vertical_tunnel(previous.y, current.y, current.x);
            } else {
                self.apply_vertical_tunnel(previous.y, current.y, previous.x);
                self.apply_horizontal_tunnel(previous.x, current.x, current.y);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, start_y: i32, end_y: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(start_y, end_y)..=max(start_y, end_y) {
            self.map.set_tile(&Point::new(x, y), FLOOR);
        }
    }

    fn apply_horizontal_tunnel(&mut self, start_x: i32, end_x: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(start_x, end_x)..=max(start_x, end_x) {
            self.map.set_tile(&Point::new(x, y), FLOOR);
        }
    }
}

impl From<&mut RandomNumberGenerator> for MapBuilder {
    fn from(rng: &mut RandomNumberGenerator) -> Self {
        let mut result = MapBuilder {
            map: Map::default(),
            rooms: Vec::new(),
            starting_point: Point::zero(),
        };
        result.fill(WALL);
        result.build_random_rooms(rng);
        result.build_corridors(rng);
        result.starting_point = result.rooms[0].center();
        result
    }
}
