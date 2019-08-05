use specs::prelude::*;

#[derive(Debug)]
pub struct Solid;

impl Default for Solid {
    fn default() -> Self {
        Solid {}
    }
}

impl Component for Solid {
    type Storage = NullStorage<Self>;
}
