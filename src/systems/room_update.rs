use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::config::DOOR_SLIDE;
use crate::resources::room::Room;
use crate::resources::room::RoomObject::Door;

pub struct RoomUpdateSystem;

impl<'a> System<'a> for RoomUpdateSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pose>,
        Write<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, poses, mut room) = data;

        if let Some((_, player_pose)) = (&players, &poses).join().next() {
            update_room(player_pose, &mut room);
        }
    }
}

fn update_room(player_pose: &Pose, room: &mut Room) {
    room.mut_cells(|cell_pos, cell| {
        if let Door {
            closing,
            closed,
            open_timer,
            ..
        } = &mut cell.object
        {
            if *closing && *closed < 1. {
                *closed = 1_f32.min(*closed + DOOR_SLIDE);
            }
            if !*closing {
                if *closed > 0. {
                    *closed = 0_f32.max(*closed - DOOR_SLIDE);
                    *open_timer = 60;
                }
                if outside_cell(player_pose.position, cell_pos) {
                    if *open_timer > 0 {
                        *open_timer -= 1;
                    } else {
                        *closing = true;
                    }
                } else {
                    *open_timer = 60;
                }
            }
            cell.blocking = *closed >= 0.5;
        }
    })
}

fn outside_cell(player_pos: Vector, cell_pos: (u32, u32)) -> bool {
    let cell = Vector::from(cell_pos) + Vector::new(0.5, 0.5);
    player_pos.distance(cell) > 1.
}
