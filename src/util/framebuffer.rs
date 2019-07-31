use quicksilver::graphics::{Image, PixelFormat};
use quicksilver::Result;

#[derive(Clone, Copy, Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn darken(&self) -> RGB {
        RGB(self.0 / 2, self.1 / 2, self.2 / 2)
    }
}

pub const PIXEL_SIZE: usize = 3;

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
            data: vec![0; width as usize * height as usize * PIXEL_SIZE],
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: RGB) {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        let offset = (self.width * y + x) as usize * PIXEL_SIZE;
        self.data[offset] = color.0;
        self.data[offset + 1] = color.1;
        self.data[offset + 2] = color.2;
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
