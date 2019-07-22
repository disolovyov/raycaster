use crate::rect::Rect;
use sdl2::pixels::Color;

pub fn new_framebuffer(width: usize, height: usize) -> Rect<Color> {
    let red = Color::RGB(255, 255, 255);
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
}
