use sdl2::pixels::Color;

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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_data(&self) -> &[u8] {
        &self.data
    }

    pub fn pitch(&self) -> usize {
        self.width * 3
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        let offset = (x + y * self.width) * 3;
        self.data[offset] = color.r;
        self.data[offset + 1] = color.g;
        self.data[offset + 2] = color.b;
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for cy in y..(y + height) {
            for cx in x..(x + width) {
                self.draw_pixel(cx, cy, color);
            }
        }
    }
}
