use quicksilver::prelude::*;

pub trait TransformExt {
    fn scale_ratio(scale: impl Scalar) -> Transform;
}

impl TransformExt for Transform {
    fn scale_ratio(scale: impl Scalar) -> Transform {
        Transform::scale(Vector::new(scale, scale))
    }
}
