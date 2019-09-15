use quicksilver::prelude::*;

#[derive(Default)]
pub struct Room {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
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

    pub fn mut_cells<F>(&mut self, f: F)
    where
        F: Fn((u32, u32), &mut Cell) -> (),
    {
        for y in 0..self.width {
            for x in 0..self.height {
                let pos = (x, y);
                f(pos, self.cell_at_mut(pos))
            }
        }
    }
}

pub trait CellAt<T: Copy> {
    fn cell_at(&self, position: T) -> &Cell;
    fn cell_at_mut(&mut self, position: T) -> &mut Cell;
    fn in_bounds(&self, position: T) -> bool;
}

impl CellAt<Vector> for Room {
    fn cell_at(&self, position: Vector) -> &Cell {
        let x = position.x as u32;
        let y = position.y as u32;
        self.cell_at((x, y))
    }

    fn cell_at_mut(&mut self, position: Vector) -> &mut Cell {
        let x = position.x as u32;
        let y = position.y as u32;
        self.cell_at_mut((x, y))
    }

    fn in_bounds(&self, position: Vector) -> bool {
        let x = position.x as u32;
        let y = position.y as u32;
        self.in_bounds((x, y))
    }
}

impl CellAt<(u32, u32)> for Room {
    fn cell_at(&self, position: (u32, u32)) -> &Cell {
        let (x, y) = position;
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        &self.cells[(self.width * y + x) as usize]
    }

    fn cell_at_mut(&mut self, position: (u32, u32)) -> &mut Cell {
        let (x, y) = position;
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        &mut self.cells[(self.width * y + x) as usize]
    }

    fn in_bounds(&self, position: (u32, u32)) -> bool {
        let (x, y) = position;
        x < self.width && y < self.height
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub ceiling: u8,
    pub floor: u8,
    pub blocking: bool,
    pub object: RoomObject,
}

#[derive(Debug, Clone)]
pub enum RoomObject {
    Empty,
    Wall {
        tile: u8,
    },
    Door {
        tile: u8,
        closing: bool,
        closed: f32,
        open_timer: u8,
    },
}
