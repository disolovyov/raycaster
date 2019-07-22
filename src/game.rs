use std::f32::consts::PI;

use sdl2::pixels::Color;

use crate::framebuffer::new_framebuffer;
use crate::map::{new_map, wall_colors};
use crate::rect::Rect;

pub struct Game {
    vw: usize,
    vh: usize,
    angle: f32,
}

impl Game {
    pub fn new(vw: usize, vh: usize) -> Game {
        Game {
            vw,
            vh,
            angle: 1.523,
        }
    }

    pub fn update(&mut self) {
        self.angle += 5.0 * PI / 360.0;
    }

    pub fn draw(&self) -> Rect<Color> {
        let mut framebuffer = new_framebuffer(self.vw * 2, self.vh);

        let wall_colors = wall_colors();

        let map = new_map();
        let rect_width = self.vw / map.width();
        let rect_height = self.vh / map.height();
        for y in 0..map.height() {
            for x in 0..map.width() {
                let cell = map.get(x, y);
                if cell == b' ' {
                    continue;
                }
                let rect_x = x * rect_width;
                let rect_y = y * rect_height;
                framebuffer.draw_rect(
                    rect_x,
                    rect_y,
                    rect_width,
                    rect_height,
                    *wall_colors.get(&cell).unwrap(),
                );
            }
        }

        let player_x: f32 = 3.456;
        let player_y: f32 = 2.345;
        let fov: f32 = PI / 3.0;
        let player_color = Color::RGB(128, 128, 128);

        framebuffer.draw_rect(
            (player_x * rect_width as f32) as usize - 2,
            (player_y * rect_height as f32) as usize - 2,
            5,
            5,
            player_color,
        );

        for ray in 0..self.vw {
            let angle = self.angle - fov / 2.0 + fov * ray as f32 / self.vw as f32;

            for i in 0..(map.width() * map.height()) {
                let t = i as f32 * 0.05;
                let cx = player_x + t * angle.cos();
                let cy = player_y + t * angle.sin();

                let pix_x = (cx * rect_width as f32) as usize;
                let pix_y = (cy * rect_height as f32) as usize;
                framebuffer.draw_pixel(pix_x, pix_y, player_color);

                let cell = map.get(cx as usize, cy as usize);
                if cell != b' ' {
                    let column_height = (self.vw as f32 / t) as usize;
                    framebuffer.draw_rect(
                        self.vw + ray,
                        (self.vh - column_height) / 2,
                        1,
                        column_height,
                        *wall_colors.get(&cell).unwrap(),
                    );
                    break;
                }
            }
        }

        framebuffer
    }
}
