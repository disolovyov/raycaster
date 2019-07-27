use quicksilver::prelude::*;

use crate::config::{VH, VW};
use crate::states::main::MainState;

mod assets;
mod components;
mod config;
mod entities;
mod gfx;
mod resources;
mod states;
mod systems;

fn main() {
    let size = Vector::new(VW, VH);
    let settings = Settings::default();
    run::<MainState>("tinyraycaster", size, settings);
}
