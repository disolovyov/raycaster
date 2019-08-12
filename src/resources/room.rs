use std::collections::HashSet;

use quicksilver::prelude::*;

pub struct Room {
    width: u32,
    height: u32,
    ceiling: u8,
    floor: u8,
    tiles: Vec<u8>,
    doors: HashSet<u8>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            width: 0,
            height: 0,
            ceiling: 0,
            floor: 0,
            tiles: vec![],
            doors: HashSet::new(),
        }
    }
}

impl Room {
    pub fn set_map(
        &mut self,
        width: u32,
        height: u32,
        ceiling: u8,
        floor: u8,
        tiles: &[u8],
        doors: &[u8],
    ) {
        self.width = width;
        self.height = height;
        self.ceiling = ceiling;
        self.floor = floor;
        self.tiles = tiles.to_vec();
        self.doors = doors.iter().cloned().collect();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn ceiling(&self) -> u8 {
        self.ceiling
    }

    pub fn floor(&self) -> u8 {
        self.floor
    }

    pub fn is_door(&self, tile: u8) -> bool {
        self.doors.contains(&tile)
    }

    pub fn get_tile(&self, position: Vector) -> u8 {
        let x = position.x as u32;
        let y = position.y as u32;
        self.get_tile_xy(x, y)
    }

    pub fn get_tile_xy(&self, x: u32, y: u32) -> u8 {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.tiles[(self.width * y + x) as usize]
    }
}
