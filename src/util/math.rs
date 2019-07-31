use quicksilver::prelude::*;

pub fn transform_scale(scalar: impl Scalar) -> Transform {
    Transform::scale(Vector::new(scalar, scalar))
}
