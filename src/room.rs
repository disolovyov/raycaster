use std::collections::HashMap;

use crate::framebuffer::RGB;

pub struct Room {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Room {
    pub fn new() -> Room {
        let map = b"\
            0000222222220000\
            1              0\
            1      11111   0\
            1     0        0\
            0     0  1110000\
            0     3        0\
            0   10000      0\
            0   0   11100  0\
            0   0   0      0\
            0   0   1  00000\
            0       1      0\
            2       1      0\
            0       0      0\
            0 0000000      0\
            0              0\
            0002222222200000";

        Room {
            width: 16,
            height: 16,
            data: map.to_vec(),
        }
    }

    pub fn wall_colors() -> HashMap<u8, RGB> {
        [
            (b'0', RGB(64, 64, 64)),
            (b'1', RGB(96, 96, 96)),
            (b'2', RGB(128, 128, 128)),
            (b'3', RGB(160, 160, 160)),
        ]
        .iter()
        .cloned()
        .collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * self.width]
    }
}
