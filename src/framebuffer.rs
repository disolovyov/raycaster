use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let red = Color::unpack(255);
        Framebuffer {
            width,
            height,
            data: vec![red; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.data[x + y * self.width] = color;
    }
}
