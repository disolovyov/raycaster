use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::mob::Mob;
use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::components::prop::Prop;
use crate::components::sprite::Sprite;

pub fn create_player(world: &mut World, position: Vector) -> Entity {
    world
        .create_entity()
        .with(Player::default())
        .with(Pose::new(position))
        .build()
}

pub fn create_prop(world: &mut World, sprite: Sprite, position: Vector) -> Entity {
    world
        .create_entity()
        .with(Prop::default())
        .with(sprite)
        .with(Pose::new(position))
        .build()
}

pub fn create_mob(world: &mut World, sprite: Sprite, position: Vector) -> Entity {
    world
        .create_entity()
        .with(Mob::new(position))
        .with(sprite)
        .with(Pose::new(position))
        .build()
}
