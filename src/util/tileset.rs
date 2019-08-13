use image::GenericImageView;

use crate::util::rgb::RGB;

pub struct Tileset {
    tile_width: u32,
    tile_height: u32,
    tile_x_count: u32,
    data: Vec<u8>,
}

impl Tileset {
    pub fn new(tile_width: u32, tile_height: u32, bytes: &[u8]) -> Tileset {
        let tex_image = image::load_from_memory(bytes).expect("Failed to load texture");
        Tileset {
            tile_width,
            tile_height,
            tile_x_count: tex_image.width() / tile_width,
            data: tex_image.to_rgba().into_raw(),
        }
    }

    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }

    pub fn get_pixel(&self, tile: u8, x: u32, y: u32) -> Option<RGB> {
        if x >= self.tile_width || y >= self.tile_height {
            return None;
        }

        let tile_index = u32::from(tile);
        let tex_x = tile_index % self.tile_x_count * self.tile_width + x;
        let tex_y = tile_index / self.tile_x_count * self.tile_height + y;
        let offset = (tex_y * self.tile_x_count * self.tile_width + tex_x) as usize * 4;
        if offset + 3 >= self.data.len() {
            return None;
        }

        let alpha = self.data[offset + 3];
        if alpha == 0 {
            return None;
        }

        Some(RGB::new(
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
        ))
    }
}
