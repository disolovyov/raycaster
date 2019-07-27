use std::f32::consts::PI;

use quicksilver::graphics::Font;
use quicksilver::input::Key;
use quicksilver::lifecycle::{Asset, State, Window};
use quicksilver::prelude::{Background::Img, Color, FontStyle};
use quicksilver::Result;

use crate::config::{VH, VW};
use crate::framebuffer::{Framebuffer, RGB};
use crate::room::Room;

struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

pub struct Game {
    player: Player,
    font: Asset<Font>,
}

impl State for Game {
    fn new() -> Result<Self> {
        Ok(Game {
            player: Player {
                x: 1.5,
                y: 1.5,
                angle: 0.0,
            },
            font: Asset::new(Font::load("font.ttf")),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let keyboard = window.keyboard();
        if keyboard[Key::A].is_down() {
            self.player.angle -= 0.03;
        } else if keyboard[Key::D].is_down() {
            self.player.angle += 0.03;
        }
        if keyboard[Key::W].is_down() {
            self.player.x += self.player.angle.cos() * 0.05;
            self.player.y += self.player.angle.sin() * 0.05;
        } else if keyboard[Key::S].is_down() {
            self.player.x -= self.player.angle.cos() * 0.05;
            self.player.y -= self.player.angle.sin() * 0.05;
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let mut framebuffer = Framebuffer::new(VW * 2, VH);

        let wall_colors = Room::wall_colors();

        let room = Room::new();
        let rect_width = VW / room.width();
        let rect_height = VH / room.height();
        for y in 0..room.height() {
            for x in 0..room.width() {
                let cell = room.get(x, y);
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

        let fov: f32 = PI / 3.0;
        let player_color = RGB(192, 192, 192);

        for ray in 0..VW {
            let angle = self.player.angle - fov / 2.0 + fov * ray as f32 / VW as f32;

            for i in 0..2000 {
                let t = i as f32 * 0.01;
                let cx = self.player.x + t * angle.cos();
                let cy = self.player.y + t * angle.sin();

                let pix_x = (cx * rect_width as f32) as usize;
                let pix_y = (cy * rect_height as f32) as usize;
                framebuffer.draw_pixel(pix_x, pix_y, player_color);

                let cell = room.get(cx as usize, cy as usize);
                if cell != b' ' {
                    let mut column_height =
                        (VW as f32 / (t * (angle - self.player.angle).cos())) as usize;
                    if column_height > VH {
                        column_height = VH;
                    }
                    framebuffer.draw_rect(
                        VW + ray,
                        (VH - column_height) / 2,
                        1,
                        column_height,
                        *wall_colors.get(&cell).unwrap(),
                    );
                    break;
                }
            }
        }

        let frame = framebuffer.to_image()?;
        window.clear(Color::BLACK)?;
        window.draw(&frame.area(), Img(&frame));

        self.font.execute(|font| {
            let fps = window.current_fps();
            let fps_counter = format!("fps: {}", fps as u8);
            let image = font.render(&fps_counter, &FontStyle::new(20.0, Color::YELLOW))?;
            window.draw(&image.area(), Img(&image));
            Ok(())
        })?;

        Ok(())
    }
}
