use std::ops::Index;

use crate::util::tileset::Tileset;

const TILES64: &[u8] = include_bytes!("../../include/64.png");
const TILES48: &[u8] = include_bytes!("../../include/48.png");

pub struct Tilesets {
    tiles64: Tileset,
    tiles48: Tileset,
}

impl Default for Tilesets {
    fn default() -> Self {
        Tilesets {
            tiles64: Tileset::new(64, 64, TILES64),
            tiles48: Tileset::new(48, 48, TILES48),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TilesetType {
    Tiles64,
    Tiles48,
}

impl Index<TilesetType> for Tilesets {
    type Output = Tileset;

    fn index(&self, index: TilesetType) -> &Self::Output {
        match index {
            TilesetType::Tiles64 => &self.tiles64,
            TilesetType::Tiles48 => &self.tiles48,
        }
    }
}
