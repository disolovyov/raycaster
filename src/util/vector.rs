use quicksilver::prelude::*;

pub trait VectorExt {
    fn angle_to(self, other: Vector) -> f32;
    fn signum(self) -> Vector;
    fn trunc(self) -> Vector;
}

impl VectorExt for Vector {
    fn angle_to(self, other: Vector) -> f32 {
        let delta = other - self;
        delta.y.atan2(delta.x)
    }

    fn signum(self) -> Vector {
        Vector::new(self.x.signum(), self.y.signum())
    }

    fn trunc(self) -> Vector {
        Vector::new(self.x.trunc(), self.y.trunc())
    }
}
