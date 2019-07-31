use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;
use crate::config::{TURN_SPEED, WALK_SPEED};
use crate::resources::input::{Binding, Input};

pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Pose>,
        Read<'a, Input>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut poses, input) = data;

        if let Some((_, pose)) = (&players, &mut poses).join().next() {
            PlayerInputSystem::handle_input(pose, &input);
        }
    }
}

impl PlayerInputSystem {
    fn handle_input(pose: &mut Pose, input: &Input) {
        if input.is_down(Binding::MoveForward) {
            pose.forward(WALK_SPEED);
        } else if input.is_down(Binding::MoveBack) {
            pose.back(WALK_SPEED);
        }
        if input.is_down(Binding::TurnLeft) {
            pose.left(TURN_SPEED);
        } else if input.is_down(Binding::TurnRight) {
            pose.right(TURN_SPEED);
        }
    }
}
