use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::sprite::{Sprite, SpriteAlign};
use crate::entities::{create_player, create_prop};
use crate::resources::room::Room;
use crate::resources::tilesets::TilesetType;

#[rustfmt::skip]
pub const DEBUG_MAP: [u8; 256] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

pub fn load_map(world: &mut World) {
    world.write_resource::<Room>().set_map(16, 16, &DEBUG_MAP);

    create_player(world);

    create_prop(
        world,
        Sprite::new(TilesetType::Sprites64, vec![13], SpriteAlign::BOTTOM),
        Vector::new(6.5, 1.5),
    );
    create_prop(
        world,
        Sprite::new(TilesetType::Sprites48, vec![10], SpriteAlign::BOTTOM),
        Vector::new(6.5, 2.5),
    );
    create_prop(
        world,
        Sprite::new(TilesetType::Sprites48, vec![2], SpriteAlign::TOP),
        Vector::new(6.5, 3.5),
    );
}
