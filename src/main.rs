use crate::framebuffer::Framebuffer;

mod color;
mod framebuffer;

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    let _framebuffer = Framebuffer::new(WIDTH, HEIGHT);
}
