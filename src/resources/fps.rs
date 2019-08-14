use quicksilver::prelude::*;

use crate::util::ringbuffer::RingBuffer;

const AVERAGE_WINDOW: usize = 60;

pub struct FPS {
    history: RingBuffer<f64>,
}

impl Default for FPS {
    fn default() -> Self {
        FPS {
            history: RingBuffer::new(0., AVERAGE_WINDOW),
        }
    }
}

impl FPS {
    pub fn update(&mut self, window: &Window) {
        let current = window.current_fps();
        self.history.push(current);
    }

    pub fn current(&self) -> f64 {
        let items = self.history.items();
        items.iter().sum::<f64>() / items.len() as f64
    }
}
