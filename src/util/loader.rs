use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::sprite::{Sprite, SpriteAlign};
use crate::entities::{create_player, create_prop};
use crate::resources::room::Room;
use crate::resources::tilesets::TilesetType;

#[rustfmt::skip]
pub const DEBUG_MAP: [u8; 256] = [
    4, 4, 4, 4, 4,  4, 4, 4, 4, 4,  4, 4, 4, 4, 4, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 21, 4, 4, 3, 4, 21, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  4, 0, 0, 0, 0,  4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  4, 0, 0, 0, 0,  3, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  3, 0, 0, 0, 0,  4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  4, 0, 0, 0, 0,  4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 21, 4, 3, 4, 4, 21, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 4,
    4, 4, 4, 4, 4,  4, 4, 4, 4, 4,  4, 4, 4, 4, 4, 4,
];

pub const DEBUG_DOORS: [u8; 1] = [3];

pub fn load_map(world: &mut World) {
    world
        .write_resource::<Room>()
        .set_map(16, 16, 22, 37, &DEBUG_MAP, &DEBUG_DOORS);

    create_player(world);

    create_prop(
        world,
        Sprite::new(TilesetType::Sprites64, vec![13], SpriteAlign::BOTTOM),
        Vector::new(6.5, 1.5),
        true,
    );
    create_prop(
        world,
        Sprite::new(TilesetType::Sprites48, vec![10], SpriteAlign::BOTTOM),
        Vector::new(6.5, 2.5),
        true,
    );
    create_prop(
        world,
        Sprite::new(TilesetType::Sprites48, vec![2], SpriteAlign::TOP),
        Vector::new(6.5, 3.5),
        false,
    );
}
