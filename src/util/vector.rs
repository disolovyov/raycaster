use quicksilver::prelude::*;

pub trait VectorExt {
    fn trunc(self) -> Vector;
}

impl VectorExt for Vector {
    fn trunc(self) -> Vector {
        Vector::new(self.x.trunc(), self.y.trunc())
    }
}
