pub struct Rect<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Copy> Rect<T> {
    pub fn new(width: usize, height: usize, data: &[T]) -> Rect<T> {
        debug_assert!(
            data.len() == width * height,
            "invalid data length {} (should be {})",
            data.len(),
            width * height
        );

        Rect {
            width,
            height,
            data: data.to_vec(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        debug_assert!(x < self.width, "x = {} out of bounds", x);
        debug_assert!(y < self.height, "y = {} out of bounds", y);

        self.data[x + y * self.width] = value;
    }
}
