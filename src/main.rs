use crate::color::{rgb, unpack_rgba};

mod color;

fn main() {
    let c = rgb(108, 113, 196);
    println!("{:?}", unpack_rgba(c));
}
