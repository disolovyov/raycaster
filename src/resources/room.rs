use quicksilver::prelude::*;

use crate::config::PLAYER_RADIUS;

pub struct Room {
    width: u32,
    height: u32,
    map: Vec<u8>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            width: 0,
            height: 0,
            map: vec![],
        }
    }
}

impl Room {
    pub fn set_map(&mut self, width: u32, height: u32, map: &[u8]) {
        self.width = width;
        self.height = height;
        self.map = map.to_vec();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_tile(&self, position: &Vector) -> u8 {
        let x = position.x as u32;
        let y = position.y as u32;
        self.get_tile_xy(x, y)
    }

    pub fn get_tile_xy(&self, x: u32, y: u32) -> u8 {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.map[(self.width * y + x) as usize]
    }

    pub fn move_to(&self, from: Vector, delta: Vector) -> Vector {
        let mut to = from + delta;

        // Disallow out of bounds
        let width = self.width as f32;
        let height = self.height as f32;
        if to.x < 0. || to.y < 0. || to.x >= width || to.y >= height {
            return from;
        }

        // Anticipate collision with player radius
        let to_buf = Transform::translate(to)
            * Transform::rotate(delta.angle())
            * Vector::new(PLAYER_RADIUS, 0);

        // Rollback x or y on collision
        if self.get_tile_xy(to_buf.x as u32, from.y as u32) != 0 {
            to.x = from.x;
        }
        if self.get_tile_xy(from.x as u32, to_buf.y as u32) != 0 {
            to.y = from.y;
        }

        to
    }
}
