use pathfinding::directed::astar::astar;
use quicksilver::prelude::*;

use crate::resources::room::{CellAt, Room};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point(u32, u32);

const POINT_PRECISION: f32 = 100.;

impl From<Vector> for Point {
    fn from(other: Vector) -> Self {
        Point(
            (other.x * POINT_PRECISION) as u32,
            (other.y * POINT_PRECISION) as u32,
        )
    }
}

impl Point {
    fn into_vector(self) -> Vector {
        Vector::new(
            self.0 as f32 / POINT_PRECISION,
            self.1 as f32 / POINT_PRECISION,
        )
    }

    fn distance(self, to: Vector) -> u32 {
        (self.into_vector().distance(to) * POINT_PRECISION) as u32
    }
}

pub fn shortest_path(from: Vector, to: Vector, buffer: f32, room: &Room) -> Option<Vector> {
    astar(
        &from.into(),
        |p| path_successors(*p, room),
        |p| p.distance(to),
        |p| p.distance(to) < (buffer * POINT_PRECISION) as u32,
    )
    .and_then(|(path, _)| path.get(1).map(|p| p.into_vector()))
}

fn path_successors(position: Point, room: &Room) -> Vec<(Point, u32)> {
    [0, 45, 90, 135, 180, 225, 270, 315]
        .iter()
        .map(|angle| Transform::rotate(*angle) * Vector::X + position.into_vector())
        .filter(|v| sprite_fits(*v, room))
        .map(|v| (v.into(), 1))
        .collect()
}

fn sprite_fits(position: Vector, room: &Room) -> bool {
    if room.cell_at(position).blocking {
        return false;
    }
    [0, 90, 180, 270]
        .iter()
        .map(|angle| Transform::rotate(*angle) * Vector::new(0, 0.5) + position)
        .all(|p| !room.cell_at(p).blocking)
}
