use std::error::Error;

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::map::Map;

mod color;
mod framebuffer;
mod map;

fn main() -> Result<(), Box<Error>> {
    let fb_width = 512;
    let fb_height = 512;
    let mut framebuffer = Framebuffer::new(fb_width, fb_height);

    for y in 0..fb_height {
        for x in 0..fb_width {
            let r = (255 * y / fb_height) as u8;
            let g = (255 * x / fb_width) as u8;
            let b = 0;
            let color = Color::rgb(r, g, b);
            framebuffer.draw_pixel(x, y, color)
        }
    }

    let map = Map::new();
    let rect_width = fb_width / map.width();
    let rect_height = fb_height / map.height();
    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y) == b' ' {
                continue;
            }
            let rect_x = x * rect_width;
            let rect_y = y * rect_height;
            let color = Color::rgb(0, 255, 255);
            framebuffer.draw_rect(rect_x, rect_y, rect_width, rect_height, color);
        }
    }

    framebuffer.write_ppm("target/out.ppm")?;

    Ok(())
}
