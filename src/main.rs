extern crate sdl2;

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

use crate::game::Game;

mod framebuffer;
mod game;
mod map;
mod rect;

const VW: usize = 512;
const VH: usize = 512;

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut canvas = video_subsystem
        .window("tinyraycaster", (VW * 2) as u32, VH as u32)
        .position_centered()
        .opengl()
        .build()?
        .into_canvas()
        .build()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let frame = Duration::new(0, 1_000_000_000u32 / 60);
    let mut event_pump = sdl_context.event_pump()?;
    let mut game = Game::new(VW, VH);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                _ => {}
            }
        }

        game.update();

        let framebuffer = game.draw();
        for y in 0..framebuffer.height() {
            for x in 0..framebuffer.width() {
                let color = framebuffer.get(x, y);
                canvas.pixel(x as i16, y as i16, color)?;
            }
        }
        canvas.present();

        sleep(frame);
    }

    Ok(())
}
