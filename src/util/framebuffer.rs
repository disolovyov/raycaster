use quicksilver::graphics::{Image, PixelFormat};
use quicksilver::Result;

use crate::util::rgb::RGB;

pub struct Framebuffer {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            data: vec![0; width as usize * height as usize * 3],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: RGB) {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        let offset = (self.width * y + x) as usize * 3;
        self.data[offset] = color.r;
        self.data[offset + 1] = color.g;
        self.data[offset + 2] = color.b;
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: RGB) {
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
