use std::collections::HashMap;

use crate::gfx::framebuffer::RGB;

pub struct Walls {
    map: HashMap<u8, RGB>,
}

impl Default for Walls {
    fn default() -> Self {
        let map = [
            (b'0', RGB(64, 64, 64)),
            (b'1', RGB(96, 96, 96)),
            (b'2', RGB(128, 128, 128)),
            (b'3', RGB(160, 160, 160)),
        ]
        .iter()
        .cloned()
        .collect();

        Walls { map }
    }
}

impl Walls {
    pub fn get_color(&self, wall: u8) -> RGB {
        *self.map.get(&wall).unwrap_or(&RGB(192, 192, 192))
    }
}
