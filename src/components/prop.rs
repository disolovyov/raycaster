use specs::prelude::*;

#[derive(Default)]
pub struct Prop;

impl Component for Prop {
    type Storage = NullStorage<Self>;
}
