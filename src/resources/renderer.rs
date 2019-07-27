use quicksilver::prelude::*;

use crate::assets::Assets;
use crate::gfx::framebuffer::Framebuffer;

pub struct Renderer {
    items: Vec<RenderItem>,
}

pub struct RenderItem {
    pub renderable: Renderable,
    pub position: Vector,
    pub layer: Layer,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    FOV,
    UI,
}

impl RenderItem {
    fn render(&self, window: &mut Window, assets: &mut Assets) -> Result<()> {
        match &self.renderable {
            Renderable::Framebuffer(framebuffer) => {
                let image = framebuffer.to_image()?;
                let rect = image.area().translate(self.position);
                window.draw(&rect, Img(&image));
                Ok(())
            }
            Renderable::Text(text, font_style) => {
                assets.draw_text(window, &self.position, &text, &font_style)
            }
        }
    }
}

pub enum Renderable {
    Framebuffer(Framebuffer),
    Text(String, FontStyle),
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer { items: vec![] }
    }
}

impl Renderer {
    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn add(&mut self, item: RenderItem) {
        self.items.push(item);
    }

    pub fn render(&mut self, window: &mut Window, assets: &mut Assets) -> Result<()> {
        self.items.sort_by(|a, b| a.layer.cmp(&b.layer));

        window.clear(Color::BLACK)?;
        for item in self.items.iter() {
            let _ = item.render(window, assets);
        }

        Ok(())
    }
}
