use quicksilver::prelude::*;

pub struct Assets {
    font: Asset<Font>,
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            font: Asset::new(Font::load("font.ttf")),
        }
    }

    pub fn draw_text(
        &mut self,
        window: &mut Window,
        position: Vector,
        text: &str,
        font_style: &FontStyle,
    ) -> Result<()> {
        self.font.execute(|font| {
            let image = font.render(text, font_style)?;
            let rect = image.area().translate(position);
            window.draw(&rect, Img(&image));
            Ok(())
        })
    }
}
