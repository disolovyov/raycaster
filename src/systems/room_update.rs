use specs::prelude::*;

use crate::config::DOOR_SLIDE;
use crate::resources::room::Room;
use crate::resources::room::RoomObject::{Door, Empty};

pub struct RoomUpdateSystem;

impl<'a> System<'a> for RoomUpdateSystem {
    type SystemData = (Write<'a, Room>,);

    fn run(&mut self, data: Self::SystemData) {
        let (mut room,) = data;
        room.mut_cells(|cell| {
            if let Door { closed, .. } = &mut cell.object {
                if *closed <= 0. {
                    cell.object = Empty;
                } else if *closed < 1. {
                    *closed -= DOOR_SLIDE;
                    if *closed <= 0.5 {
                        cell.blocking = false;
                    }
                }
            }
        })
    }
}
