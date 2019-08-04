use std::ops::Index;

use crate::util::tileset::Tileset;

const WALLS: &[u8] = include_bytes!("../../include/walls.png");
const SPRITES64: &[u8] = include_bytes!("../../include/sprites64.png");

pub struct Tilesets {
    walls: Tileset,
    sprites64: Tileset,
}

impl Default for Tilesets {
    fn default() -> Self {
        Tilesets {
            walls: Tileset::new(64, 64, WALLS),
            sprites64: Tileset::new(64, 64, SPRITES64),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TilesetType {
    Walls,
    Sprites64,
}

impl Index<TilesetType> for Tilesets {
    type Output = Tileset;

    fn index(&self, index: TilesetType) -> &Self::Output {
        match index {
            TilesetType::Walls => &self.walls,
            TilesetType::Sprites64 => &self.sprites64,
        }
    }
}
