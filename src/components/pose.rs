use quicksilver::prelude::*;
use specs::prelude::*;

use crate::util::ext::transform::TransformExt;

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

    pub fn move_forward(&self, delta: f32) -> Vector {
        Transform::scale_ratio(delta) * self.direction
    }

    pub fn move_back(&self, delta: f32) -> Vector {
        Transform::scale_ratio(delta) * -self.direction
    }

    pub fn strafe_left(&self, delta: f32) -> Vector {
        Transform::scale_ratio(delta) * Transform::rotate(-90) * self.direction
    }

    pub fn strafe_right(&self, delta: f32) -> Vector {
        Transform::scale_ratio(delta) * Transform::rotate(90) * self.direction
    }

    pub fn turn_left(&mut self, delta: f32) {
        self.direction = Transform::rotate(-delta) * self.direction
    }

    pub fn turn_right(&mut self, delta: f32) {
        self.direction = Transform::rotate(delta) * self.direction
    }
}
