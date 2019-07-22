pub struct Map {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            width: 16,
            height: 16,
            data: {
                b"\
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
                0002222222200000"
                    .to_vec()
            },
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width, "x = {} out of bounds", x);
        assert!(y < self.height, "y = {} out of bounds", y);
        self.data[x + y * self.width]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
