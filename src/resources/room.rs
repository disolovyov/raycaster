use quicksilver::prelude::*;

pub struct Room {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            width: 0,
            height: 0,
            cells: vec![],
        }
    }
}

impl Room {
    pub fn set_map(&mut self, width: u32, height: u32, cells: &[Cell]) {
        self.width = width;
        self.height = height;
        self.cells = cells.to_vec();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub trait CellAt<T> {
    fn cell_at(&self, position: T) -> Cell;
}

impl CellAt<Vector> for Room {
    fn cell_at(&self, position: Vector) -> Cell {
        let x = position.x as u32;
        let y = position.y as u32;
        self.cell_at((x, y))
    }
}

impl CellAt<(u32, u32)> for Room {
    fn cell_at(&self, position: (u32, u32)) -> Cell {
        let (x, y) = position;
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.cells[(self.width * y + x) as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub ceiling: u8,
    pub floor: u8,
    pub blocking: bool,
    pub object: RoomObject,
}

#[derive(Debug, Clone, Copy)]
pub enum RoomObject {
    Empty,
    Wall { tile: u8 },
    Door { tile: u8, closed: f32 },
}
