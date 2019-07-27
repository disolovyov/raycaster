use quicksilver::prelude::*;

#[derive(Default)]
pub struct FPS {
    current: f64,
}

impl FPS {
    pub fn update(&mut self, window: &Window) {
        self.current = window.current_fps();
    }

    pub fn current(&self) -> f64 {
        self.current
    }
}
