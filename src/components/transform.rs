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

    pub fn move_forward(&mut self, delta: f32, room: &Room) {
        let delta = Transform::scale_ratio(delta) * self.direction;
        self.move_to(delta, room);
    }

    pub fn move_back(&mut self, delta: f32, room: &Room) {
        let delta = Transform::scale_ratio(delta) * -self.direction;
        self.move_to(delta, room);
    }

    pub fn strafe_left(&mut self, delta: f32, room: &Room) {
        let delta = Transform::scale_ratio(delta) * Transform::rotate(90) * self.direction;
        self.move_to(delta, room);
    }

    pub fn strafe_right(&mut self, delta: f32, room: &Room) {
        let delta = Transform::scale_ratio(delta) * Transform::rotate(-90) * self.direction;
        self.move_to(delta, room);
    }

    fn move_to(&mut self, delta: Vector, room: &Room) {
        let target = self.position + delta;
        if !room.is_solid(target.x as u32, target.y as u32) {
            self.position = target;
        }
    }

    pub fn turn_left(&mut self, delta: f32) {
        self.direction = Transform::rotate(delta) * self.direction
    }

    pub fn turn_right(&mut self, delta: f32) {
        self.direction = Transform::rotate(-delta) * self.direction
    }
}
