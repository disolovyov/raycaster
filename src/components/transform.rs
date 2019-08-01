use quicksilver::prelude::*;
use specs::prelude::*;

use crate::resources::room::Room;
use crate::util::transform::TransformExt;

#[derive(Debug)]
pub struct Pose {
    pub position: Vector,
    pub direction: Vector,
}

impl Component for Pose {
    type Storage = VecStorage<Self>;
}

impl Pose {
    pub fn new(position: Vector) -> Pose {
        Pose {
            position,
            direction: Vector::new(1, 0),
        }
    }

    pub fn left(&mut self, delta: f32) {
        self.direction = Transform::rotate(delta) * self.direction
    }

    pub fn right(&mut self, delta: f32) {
        self.direction = Transform::rotate(-delta) * self.direction
    }

    pub fn forward(&mut self, delta: f32, room: &Room) {
        let position = self.position + Transform::scale_ratio(delta) * self.direction;
        if !room.is_solid(position.x as u32, position.y as u32) {
            self.position = position;
        }
    }

    pub fn back(&mut self, delta: f32, room: &Room) {
        let position = self.position - Transform::scale_ratio(delta) * self.direction;
        if !room.is_solid(position.x as u32, position.y as u32) {
            self.position = position;
        }
    }
}
