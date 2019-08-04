#[derive(Clone, Copy, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }

    pub fn darken(&self) -> RGB {
        RGB {
            r: self.r / 2,
            g: self.g / 2,
            b: self.b / 2,
        }
    }
}
