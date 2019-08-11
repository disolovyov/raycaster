#![allow(clippy::match_bool, clippy::type_complexity)]

use quicksilver::prelude::*;

use crate::config::{VH, VW};
use crate::states::main::MainState;

mod assets;
mod components;
mod config;
mod entities;
mod resources;
mod states;
mod systems;
mod util;

fn main() {
    let size = Vector::new(VW, VH);
    let settings = Settings::default();
    run::<MainState>("tinyraycaster", size, settings);
}
