pub struct RingBuffer<T: Clone> {
    pos: usize,
    capacity: usize,
    length: usize,
    items: Vec<T>,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(initial: T, capacity: usize) -> RingBuffer<T> {
        RingBuffer {
            pos: 0,
            capacity,
            length: 0,
            items: vec![initial; capacity],
        }
    }

    pub fn push(&mut self, item: T) {
        self.items[self.pos] = item;
        self.length = self.capacity.min(self.length + 1);
        self.pos = (self.pos + 1) % self.capacity;
    }

    pub fn items(&self) -> &[T] {
        &self.items[..self.length]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push() {
        let mut buffer = RingBuffer::new(0, 3);

        assert_eq!(buffer.items(), &[]);

        buffer.push(1);
        buffer.push(2);

        assert_eq!(buffer.items(), &[1, 2]);

        buffer.push(3);

        assert_eq!(buffer.items(), &[1, 2, 3]);

        buffer.push(4);
        buffer.push(5);

        assert_eq!(buffer.items(), &[4, 5, 3]);
    }
}
