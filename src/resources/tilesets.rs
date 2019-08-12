use std::ops::Index;

use crate::util::tileset::Tileset;

const ROOM: &[u8] = include_bytes!("../../include/room.png");
const SPRITES64: &[u8] = include_bytes!("../../include/sprites64.png");
const SPRITES48: &[u8] = include_bytes!("../../include/sprites48.png");

pub struct Tilesets {
    room: Tileset,
    sprites64: Tileset,
    sprites48: Tileset,
}

impl Default for Tilesets {
    fn default() -> Self {
        Tilesets {
            room: Tileset::new(64, 64, ROOM),
            sprites64: Tileset::new(64, 64, SPRITES64),
            sprites48: Tileset::new(48, 48, SPRITES48),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TilesetType {
    Room,
    Sprites64,
    Sprites48,
}

impl Index<TilesetType> for Tilesets {
    type Output = Tileset;

    fn index(&self, index: TilesetType) -> &Self::Output {
        match index {
            TilesetType::Room => &self.room,
            TilesetType::Sprites64 => &self.sprites64,
            TilesetType::Sprites48 => &self.sprites48,
        }
    }
}
