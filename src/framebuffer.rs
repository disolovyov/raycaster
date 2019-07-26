use quicksilver::graphics::{Image, PixelFormat};
use quicksilver::Result;

#[derive(Clone, Copy, Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

pub struct Framebuffer {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            width,
            height,
            data: vec![255; width * height * 3],
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: RGB) {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        let offset = (x + y * self.width) * 3;
        self.data[offset] = color.0;
        self.data[offset + 1] = color.1;
        self.data[offset + 2] = color.2;
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: RGB) {
        for cy in y..(y + height) {
            for cx in x..(x + width) {
                self.draw_pixel(cx, cy, color);
            }
        }
    }

    pub fn to_image(&self) -> Result<Image> {
        Image::from_raw(
            &self.data,
            self.width as u32,
            self.height as u32,
            PixelFormat::RGB,
        )
    }
}
