use quicksilver::prelude::*;
use specs::prelude::*;

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

    pub fn forward(&mut self, delta: f32) {
        self.position += Transform::scale(Vector::new(delta, delta)) * self.direction;
    }

    pub fn back(&mut self, delta: f32) {
        self.position -= Transform::scale(Vector::new(delta, delta)) * self.direction;
    }
}
