use specs::prelude::*;

#[derive(Debug)]
pub struct Player;

impl Default for Player {
    fn default() -> Self {
        Player {}
    }
}

impl Component for Player {
    type Storage = NullStorage<Self>;
}
