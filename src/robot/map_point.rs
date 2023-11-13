
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use super::map_direction::MapDirection;

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord, Default, Serialize, Deserialize, ToSchema)]
pub struct MapPoint {
    pub x: i64,
    pub y: i64,
}

impl MapPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn move_by_direction(&mut self, direction: &MapDirection) {
        match direction {
            MapDirection::North => self.move_up(),
            MapDirection::East => self.move_right(),
            MapDirection::South => self.move_down(),
            MapDirection::West => self.move_left(),
        };
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MapWalker {
    pub direction: MapDirection,
    pub position: MapPoint,
}

impl From<MapDirection> for MapWalker {
    fn from(direction: MapDirection) -> Self {
        Self { direction, position: MapPoint { x: 0, y: 0 } }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapInfoWalker<T> {
    pub map_walker: MapWalker,
    pub info: T,
}

impl MapPoint {
    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }
}
