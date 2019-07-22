use std::error::Error;

use crate::color::Color;
use crate::framebuffer::Framebuffer;

mod color;
mod framebuffer;

fn main() -> Result<(), Box<Error>> {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = (255 * y / HEIGHT) as u8;
            let g = (255 * x / WIDTH) as u8;
            let b = 0;
            framebuffer.set(x, y, Color::rgb(r, g, b))
        }
    }
    framebuffer.write_ppm("target/out.ppm")?;

    Ok(())
}
