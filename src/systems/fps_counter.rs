use quicksilver::prelude::*;
use specs::prelude::*;

use crate::resources::fps::FPS;
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};

pub struct FpsCounterSystem;

impl<'a> System<'a> for FpsCounterSystem {
    type SystemData = (Write<'a, Renderer>, Read<'a, FPS>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut renderer, fps) = data;

        let text = format!("FPS: {}", fps.current() as u64);
        let font_style = FontStyle::new(20.0, Color::YELLOW);
        renderer.add(RenderItem {
            renderable: Renderable::Text(text, font_style),
            position: Vector::new(10, 5),
            layer: Layer::UI,
        })
    }
}
