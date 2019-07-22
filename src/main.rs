use std::error::Error;

use crate::color::Color;
use crate::framebuffer::new_framebuffer;
use crate::map::new_map;

mod color;
mod framebuffer;
mod map;
mod rect;

fn main() -> Result<(), Box<Error>> {
    let fb_width = 512;
    let fb_height = 512;
    let mut framebuffer = new_framebuffer(fb_width, fb_height);

    for y in 0..fb_height {
        for x in 0..fb_width {
            let r = (255 * y / fb_height) as u8;
            let g = (255 * x / fb_width) as u8;
            let b = 0;
            let color = Color::rgb(r, g, b);
            framebuffer.draw_pixel(x, y, color)
        }
    }

    let map = new_map();
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

    let player_x: f64 = 3.456;
    let player_y: f64 = 2.345;
    let player_a: f64 = 1.523;
    let player_color = Color::rgb(255, 255, 255);

    framebuffer.draw_rect(
        (player_x * rect_width as f64) as usize - 2,
        (player_y * rect_height as f64) as usize - 2,
        5,
        5,
        player_color,
    );

    for i in 0..(map.width() * map.height()) {
        let t = i as f64 * 0.05;
        let cx = player_x + t * player_a.cos();
        let cy = player_y + t * player_a.sin();
        if map.get(cx as usize, cy as usize) != b' ' {
            break;
        }
        let pix_x = (cx * rect_width as f64) as usize;
        let pix_y = (cy * rect_height as f64) as usize;
        framebuffer.draw_pixel(pix_x, pix_y, player_color)
    }

    framebuffer.write_ppm("target/out.ppm")?;

    Ok(())
}
