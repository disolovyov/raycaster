use crate::rect::Rect;
use sdl2::pixels::Color;
use std::collections::HashMap;

pub fn new_map() -> Rect<u8> {
    let map = b"\
        0000222222220000\
        1              0\
        1      11111   0\
        1     0        0\
        0     0  1110000\
        0     3        0\
        0   10000      0\
        0   0   11100  0\
        0   0   0      0\
        0   0   1  00000\
        0       1      0\
        2       1      0\
        0       0      0\
        0 0000000      0\
        0              0\
        0002222222200000";

    Rect::new(16, 16, map)
}

pub fn wall_colors() -> HashMap<u8, Color> {
    [
        (b'0', Color::RGB(64, 64, 64)),
        (b'1', Color::RGB(96, 96, 96)),
        (b'2', Color::RGB(128, 128, 128)),
        (b'3', Color::RGB(160, 160, 160)),
    ]
    .iter()
    .cloned()
    .collect()
}
