use specs::prelude::*;

use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::config::{PLAYER_RADIUS, WALK_SPEED};
use crate::resources::input::{Binding, Input};
use crate::resources::room::RoomObject::Door;
use crate::resources::room::{CellAt, Room};

pub struct PlayerActionsSystem;

impl<'a> System<'a> for PlayerActionsSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pose>,
        Read<'a, Input>,
        Write<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, poses, input, mut room) = data;

        if let Some((_, player_pose)) = (&players, &poses).join().next() {
            handle_actions(player_pose, &input, &mut room);
        }
    }
}

fn handle_actions(player_pose: &Pose, input: &Input, room: &mut Room) {
    if input.is_down(Binding::Action) {
        let next_step = player_pose.position + player_pose.move_forward(PLAYER_RADIUS + WALK_SPEED);
        let cell = room.cell_at_mut(next_step);
        if let Door { closing, .. } = &mut cell.object {
            *closing = false;
        }
    }
}
