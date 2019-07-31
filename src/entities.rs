use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;

pub fn create_player(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Player::default())
        .with(Pose::new(Vector::new(2, 2)))
        .build()
}
