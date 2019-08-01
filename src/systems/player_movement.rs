use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;
use crate::config::{STRAFE_SPEED, TURN_SPEED, WALK_SPEED};
use crate::resources::input::{Binding, Input};
use crate::resources::room::Room;

pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Pose>,
        Read<'a, Input>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut poses, input, room) = data;

        if let Some((_, pose)) = (&players, &mut poses).join().next() {
            handle_input(pose, &input, &room);
        }
    }
}

fn handle_input(pose: &mut Pose, input: &Input, room: &Room) {
    if input.is_down(Binding::MoveForward) {
        pose.move_forward(WALK_SPEED, room);
    } else if input.is_down(Binding::MoveBack) {
        pose.move_back(WALK_SPEED, room);
    }

    if input.is_down(Binding::StrafeLeft) {
        pose.strafe_left(STRAFE_SPEED, room);
    } else if input.is_down(Binding::StrafeRight) {
        pose.strafe_right(STRAFE_SPEED, room);
    }

    if input.is_down(Binding::TurnLeft) {
        pose.turn_left(TURN_SPEED);
    } else if input.is_down(Binding::TurnRight) {
        pose.turn_right(TURN_SPEED);
    }
}
