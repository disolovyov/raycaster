use std::path::Path;

use crate::gfx::framebuffer::RGB;

pub struct Room {
    width: u32,
    height: u32,
    map: Vec<u8>,
}

impl Default for Room {
    fn default() -> Self {
        Room::tiled_map("test.tmx")
    }
}

impl Room {
    pub fn tiled_map(name: &str) -> Self {
        let result = tiled::parse_file(&Path::new("test.tmx"));
        let tiled_map = result.expect(&format!("Failed to load {}", name));
        let map = tiled_map.layers[0]
            .tiles
            .iter()
            .flat_map(|row| row.iter().map(|tile| *tile as u8))
            .collect();

        Room {
            width: tiled_map.width,
            height: tiled_map.height,
            map,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.map[(self.width * y + x) as usize]
    }

    pub fn get_color(&self, tile: u8) -> RGB {
        match tile {
            1 => RGB(255, 0, 255),
            2 => RGB(255, 0, 0),
            4 => RGB(128, 128, 128),
            6 => RGB(192, 192, 192),
            7 => RGB(64, 64, 0),
            _ => RGB(0, 0, 0),
        }
    }
}
