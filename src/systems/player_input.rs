use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Transform;
use crate::config::{TURN_SPEED, WALK_SPEED};
use crate::resources::input::{Binding, Input};

pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, Input>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut transforms, input) = data;

        for (_, transform) in (&players, &mut transforms).join() {
            if input.is_down(Binding::MoveForward) {
                transform.forward(WALK_SPEED);
            } else if input.is_down(Binding::MoveBack) {
                transform.back(WALK_SPEED);
            }
            if input.is_down(Binding::TurnLeft) {
                transform.left(TURN_SPEED);
            } else if input.is_down(Binding::TurnRight) {
                transform.right(TURN_SPEED);
            }
        }
    }
}
