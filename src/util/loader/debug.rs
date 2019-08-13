use crate::resources::room::Cell;
use crate::resources::room::RoomObject::{Door, Empty, Wall};

#[rustfmt::skip]
pub const DEBUG_MAP: [Cell; 256] = [
    W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1,
    W1, E0, E0, E0, E0, E0, E1, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E1, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, W2, W1, W1, D1, W1, W2, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, W1, E0, E0, E0, E0, W1, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, W1, E0, E0, E0, E0, D1, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, D1, E0, E0, E0, E0, W1, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, W1, E0, E0, E0, E0, W1, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, W2, W1, D1, W1, W1, W2, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, E0, W1,
    W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1, W1,
];

const E0: Cell = Cell {
    ceiling: 74,
    floor: 89,
    blocking: false,
    object: Empty,
};

const E1: Cell = Cell {
    ceiling: 74,
    floor: 89,
    blocking: true,
    object: Empty,
};

const W1: Cell = Cell {
    ceiling: 74,
    floor: 89,
    blocking: true,
    object: Wall { tile: 56 },
};

const W2: Cell = Cell {
    ceiling: 74,
    floor: 89,
    blocking: true,
    object: Wall { tile: 73 },
};

const D1: Cell = Cell {
    ceiling: 74,
    floor: 89,
    blocking: true,
    object: Door {
        tile: 14,
        closed: 1.,
    },
};
