use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::components::sprite::Sprite;

pub fn create_player(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Player::default())
        .with(Pose::new(Vector::new(2, 2)))
        .build()
}

pub fn create_prop(world: &mut World, sprite: Sprite, position: Vector) -> Entity {
    world
        .create_entity()
        .with(sprite)
        .with(Pose::new(position))
        .build()
}
