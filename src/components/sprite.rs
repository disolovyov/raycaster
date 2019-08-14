use specs::prelude::*;

use crate::resources::animation::Animation;
use crate::resources::tilesets::TilesetType;

#[derive(Debug, Copy, Clone)]
pub enum SpriteAlign {
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct Sprite {
    tileset: TilesetType,
    frame: u8,
    frames: Vec<u8>,
    align: SpriteAlign,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

impl Sprite {
    pub fn new(tileset: TilesetType, frames: Vec<u8>, align: SpriteAlign) -> Sprite {
        Sprite {
            tileset,
            frame: 0,
            frames,
            align,
        }
    }

    pub fn tileset(&self) -> TilesetType {
        self.tileset
    }

    pub fn tileset_id(&self) -> u8 {
        self.frames[self.frame as usize]
    }

    pub fn align(&self) -> SpriteAlign {
        self.align
    }

    pub fn animate(&mut self, animation: &Animation) {
        self.frame = animation.frame(15, self.frames.len() as u8);
    }
}
