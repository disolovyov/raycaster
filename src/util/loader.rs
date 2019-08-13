use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::sprite::{Sprite, SpriteAlign};
use crate::entities::{create_player, create_prop};
use crate::resources::room::Room;
use crate::resources::tilesets::TilesetType::{Tiles48, Tiles64};
use crate::util::loader::debug::DEBUG_MAP;

mod debug;

pub fn load_map(world: &mut World) {
    world.write_resource::<Room>().set_map(16, 16, &DEBUG_MAP);

    create_player(world);

    create_prop(
        world,
        Sprite::new(Tiles64, vec![12], SpriteAlign::Bottom),
        Vector::new(6.5, 1.5),
        true,
    );
    create_prop(
        world,
        Sprite::new(Tiles48, vec![9], SpriteAlign::Bottom),
        Vector::new(6.5, 2.5),
        true,
    );
    create_prop(
        world,
        Sprite::new(Tiles48, vec![1], SpriteAlign::Top),
        Vector::new(6.5, 3.5),
        false,
    );
}
