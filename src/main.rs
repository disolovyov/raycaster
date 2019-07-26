use quicksilver::geom::Vector;
use quicksilver::lifecycle::run;
use quicksilver::lifecycle::Settings;

use crate::config::{VH, VW};
use crate::game::Game;

mod config;
mod framebuffer;
mod game;
mod room;

fn main() {
    let size = Vector::new(VW as u32 * 2, VH as u32);
    let settings = Settings::default();
    run::<Game>("tinyraycaster", size, settings);
}
