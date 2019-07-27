use std::collections::HashMap;

use quicksilver::prelude::*;

#[derive(PartialEq, Eq, Hash)]
pub enum Binding {
    MoveForward,
    MoveBack,
    TurnLeft,
    TurnRight,
}

#[derive(Default)]
pub struct Input {
    keys: HashMap<Binding, bool>,
}

impl Input {
    pub fn update(&mut self, window: &Window) {
        let keyboard = window.keyboard();
        self.keys
            .insert(Binding::MoveForward, keyboard[Key::W].is_down());
        self.keys
            .insert(Binding::MoveBack, keyboard[Key::S].is_down());
        self.keys
            .insert(Binding::TurnLeft, keyboard[Key::A].is_down());
        self.keys
            .insert(Binding::TurnRight, keyboard[Key::D].is_down());
    }

    pub fn is_down(&self, binding: Binding) -> bool {
        *self.keys.get(&binding).unwrap_or(&false)
    }
}
