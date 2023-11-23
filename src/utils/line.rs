use serde::{Deserialize, Serialize};

use super::position::Position;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Line {
    pub start: Position,
    pub end: Position,
}

impl Line {
    pub fn intersections(&self, other: &Line) -> i64 {
        let self_is_horizontal = self.is_horizontal();
        let other_is_horizontal = other.is_horizontal();

        match (self_is_horizontal, other_is_horizontal) {
            (true, true) => {
                if self.start.y == other.start.y {
                    other.get_horizontal_overlap(self)
                } else {
                    0
                }
            }
            (false, false) => {
                if self.start.x == other.start.x {
                    self.get_vertical_overlap(other)
                } else {
                    0
                }
            }
            (true, false) => self.has_intersection(other),
            _ => other.has_intersection(self),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn has_intersection(&self, other: &Line) -> i64 {
        let this_start_x = self.start.x.min(self.end.x);
        let this_end_x = self.start.x.max(self.end.x);
        if this_start_x > other.start.x || this_end_x < other.start.x {
            return 0;
        }

        let other_start_y = other.start.y.min(other.end.y);
        let other_end_y = other.start.y.max(other.end.y);

        if other_start_y < self.start.y && self.start.y < other_end_y {
            1
        } else {
            0
        }
    }

    fn get_horizontal_overlap(&self, other: &Line) -> i64 {
        let start = self.start.x.min(self.end.x);
        let end = self.start.x.max(self.end.x);
        let other_start = other.start.x.min(other.end.x);
        let other_end = other.start.x.max(other.end.x);

        overlapping(start, end, other_start, other_end)
    }

    fn get_vertical_overlap(&self, other: &Line) -> i64 {
        let start = self.start.y.min(self.end.y);
        let end = self.start.y.max(self.end.y);
        let other_start = other.start.y.min(other.end.y);
        let other_end = other.start.y.max(other.end.y);

        overlapping(start, end, other_start, other_end)
    }
}

fn overlapping(start: i64, end: i64, other_start: i64, other_end: i64) -> i64 {
    let start_max = other_start.max(start);
    let end_min = other_end.min(end);
    let overlap = end_min - start_max;

    if overlap == 0 {
        1
    } else {
        0.max(overlap + 1)
    }
}
