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

    for (x, y) in [(2.5, 2.5), (2.5, 13.5), (13.5, 2.5), (13.5, 13.5)].iter() {
        create_prop(
            world,
            Sprite::new(Tiles48, vec![9], SpriteAlign::Bottom),
            Vector::new(*x, *y),
        );
    }

    for (x, y) in [(8., 4.5), (8.5, 4.5), (9., 4.5)].iter() {
        create_prop(
            world,
            Sprite::new(Tiles48, vec![1], SpriteAlign::Top),
            Vector::new(*x, *y),
        );
    }

    create_prop(
        world,
        Sprite::new(Tiles64, vec![37, 39, 38, 39], SpriteAlign::Bottom),
        Vector::new(8.5, 2.5),
    );
}
