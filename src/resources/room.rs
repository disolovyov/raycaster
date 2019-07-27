pub struct Room {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Default for Room {
    fn default() -> Self {
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
}

impl Room {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        self.data[(self.width * y + x) as usize]
    }
}
