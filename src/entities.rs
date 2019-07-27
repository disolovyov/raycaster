use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Transform;

pub fn create_player(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Player::default())
        .with(Transform::new(Vector::new(1.5, 1.5)))
        .build()
}
