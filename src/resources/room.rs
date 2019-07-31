use crate::util::framebuffer::{PIXEL_SIZE, RGB};

pub const TILE_COUNT: u8 = 69;
pub const TILE_LINE_COUNT: usize = 10;
pub const TILE_SIZE: usize = 32;
pub const TEXTURES_WIDTH: usize = TILE_SIZE * TILE_LINE_COUNT;

const TILED_BYTES: &[u8] = include_bytes!("../../include/test.tmx");
const TEXTURES_BYTES: &[u8] = include_bytes!("../../include/textures.png");

pub struct Room {
    width: u32,
    height: u32,
    map: Vec<u8>,
    textures: Vec<u8>,
}

impl Default for Room {
    fn default() -> Self {
        Room::tiled_map()
    }
}

impl Room {
    pub fn tiled_map() -> Self {
        let tiled_map = tiled::parse(TILED_BYTES).expect("Failed to load map");
        let map = tiled_map.layers[0]
            .tiles
            .iter()
            .flat_map(|row| row.iter().map(|tile| *tile as u8))
            .collect();

        let tex_image = image::load_from_memory(TEXTURES_BYTES).expect("Failed to load textures");
        let textures = tex_image.to_rgb().into_raw();

        Room {
            width: tiled_map.width,
            height: tiled_map.height,
            map,
            textures,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.map[(self.width * y + x) as usize]
    }

    pub fn get_color(&self, tile: u8) -> RGB {
        match tile {
            1...TILE_COUNT => self.get_texture_pixel(tile, 1, 1),
            _ => RGB(0, 0, 0),
        }
    }

    pub fn get_texture_pixel(&self, tile: u8, x: usize, y: usize) -> RGB {
        let tile_index = tile as usize - 1;
        let tex_x = tile_index % TILE_LINE_COUNT * TILE_SIZE + x;
        let tex_y = tile_index / TILE_LINE_COUNT * TILE_SIZE + y;
        let offset = (tex_y * TEXTURES_WIDTH + tex_x) * PIXEL_SIZE;
        RGB(
            self.textures[offset],
            self.textures[offset + 1],
            self.textures[offset + 2],
        )
    }
}
