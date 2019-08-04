use specs::prelude::*;

use crate::resources::tilesets::TilesetType;

#[derive(Debug)]
pub struct Sprite {
    tileset: TilesetType,
    frame: u8,
    frames: Vec<u8>,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

impl Sprite {
    pub fn new(tileset: TilesetType, frames: Vec<u8>) -> Sprite {
        Sprite {
            tileset,
            frame: 0,
            frames,
        }
    }

    pub fn tileset(&self) -> TilesetType {
        self.tileset
    }

    pub fn tileset_id(&self) -> u8 {
        self.frames[self.frame as usize]
    }
}
