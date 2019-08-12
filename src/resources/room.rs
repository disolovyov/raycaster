use quicksilver::prelude::*;

pub struct Room {
    width: u32,
    height: u32,
    ceiling: u8,
    floor: u8,
    walls: Vec<u8>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            width: 0,
            height: 0,
            ceiling: 0,
            floor: 0,
            walls: vec![],
        }
    }
}

impl Room {
    pub fn set_map(&mut self, width: u32, height: u32, ceiling: u8, floor: u8, walls: &[u8]) {
        self.width = width;
        self.height = height;
        self.ceiling = ceiling;
        self.floor = floor;
        self.walls = walls.to_vec();
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

    pub fn get_wall(&self, position: Vector) -> u8 {
        let x = position.x as u32;
        let y = position.y as u32;
        self.get_wall_xy(x, y)
    }

    pub fn get_wall_xy(&self, x: u32, y: u32) -> u8 {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.walls[(self.width * y + x) as usize]
    }
}
