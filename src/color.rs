#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn unpack(rgba: u32) -> Color {
        let r = (rgba >> 0) as u8;
        let g = (rgba >> 8) as u8;
        let b = (rgba >> 16) as u8;
        let a = (rgba >> 24) as u8;
        Color { r, g, b, a }
    }

    pub fn pack(&self) -> u32 {
        let r = self.r as u32;
        let g = (self.g as u32) << 8;
        let b = (self.b as u32) << 16;
        let a = (self.a as u32) << 24;
        r + g + b + a
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test() {
        let color = Color::rgb(108, 113, 196);
        let rgba = color.pack();
        let unpacked = Color::unpack(rgba);

        assert_eq!(color, unpacked);
    }
}
