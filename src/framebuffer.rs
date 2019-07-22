use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::color::Color;
use crate::rect::Rect;

pub fn new_framebuffer(width: usize, height: usize) -> Rect<Color> {
    let red = Color::rgb(255, 255, 255);
    let data = &vec![red; width * height];
    Rect::new(width, height, data)
}

impl Rect<Color> {
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.set(x, y, color);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for cy in y..(y + height) {
            for cx in x..(x + width) {
                self.draw_pixel(cx, cy, color);
            }
        }
    }

    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut output = File::create(path)?;
        write!(output, "P6\n{} {}\n255\n", self.width(), self.height())?;
        for color in self.iter() {
            let (r, g, b, _) = color.bytes();
            output.write_all(&[r, g, b])?;
        }
        Ok(())
    }
}
