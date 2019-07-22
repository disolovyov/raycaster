use std::error::Error;
use std::f32::consts::PI;

use crate::color::Color;
use crate::framebuffer::new_framebuffer;
use crate::map::new_map;

mod color;
mod framebuffer;
mod map;
mod rect;

fn main() -> Result<(), Box<Error>> {
    let view_width = 512;
    let view_height = 512;
    let mut framebuffer = new_framebuffer(view_width, view_height);

    let map = new_map();
    let rect_width = view_width / map.width();
    let rect_height = view_height / map.height();
    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y) == b' ' {
                continue;
            }
            let rect_x = x * rect_width;
            let rect_y = y * rect_height;
            let color = Color::rgb(0, 0, 0);
            framebuffer.draw_rect(rect_x, rect_y, rect_width, rect_height, color);
        }
    }

    let player_x: f32 = 3.456;
    let player_y: f32 = 2.345;
    let player_a: f32 = 1.523;
    let fov: f32 = PI / 3.0;
    let player_color = Color::rgb(128, 128, 128);

    framebuffer.draw_rect(
        (player_x * rect_width as f32) as usize - 2,
        (player_y * rect_height as f32) as usize - 2,
        5,
        5,
        player_color,
    );

    for ray in 0..view_width {
        let angle = player_a - fov / 2.0 + fov * ray as f32 / view_width as f32;

        for i in 0..(map.width() * map.height()) {
            let t = i as f32 * 0.05;
            let cx = player_x + t * angle.cos();
            let cy = player_y + t * angle.sin();
            if map.get(cx as usize, cy as usize) != b' ' {
                break;
            }

            let pix_x = (cx * rect_width as f32) as usize;
            let pix_y = (cy * rect_height as f32) as usize;
            framebuffer.draw_pixel(pix_x, pix_y, player_color)
        }
    }

    framebuffer.write_ppm("target/out.ppm")?;

    Ok(())
}
