use quicksilver::prelude::*;
use specs::prelude::*;

#[derive(Debug)]
pub struct Transform {
    pub position: Vector,
    pub angle: f32,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn new(position: Vector) -> Transform {
        Transform {
            position,
            angle: 0.0,
        }
    }

    pub fn left(&mut self, delta: f32) {
        self.angle -= delta;
    }

    pub fn right(&mut self, delta: f32) {
        self.angle += delta;
    }

    pub fn forward(&mut self, delta: f32) {
        self.position.x += self.angle.cos() * delta;
        self.position.y += self.angle.sin() * delta;
    }

    pub fn back(&mut self, delta: f32) {
        self.position.x -= self.angle.cos() * delta;
        self.position.y -= self.angle.sin() * delta;
    }
}
