use crate::util::tileset::Tileset;

const WALLS: &[u8] = include_bytes!("../../include/textures.png");

pub struct Tilesets {
    walls: Tileset,
}

impl Default for Tilesets {
    fn default() -> Self {
        Tilesets {
            walls: Tileset::new(64, 64, WALLS),
        }
    }
}

impl Tilesets {
    pub fn walls(&self) -> &Tileset {
        &self.walls
    }
}
