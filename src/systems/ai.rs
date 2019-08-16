use specs::prelude::*;

use crate::components::mob::Mob;
use crate::components::mob::MobMovement::FollowPlayer;
use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::resources::room::{CellAt, Room};
use crate::systems::ai::pathfinding::shortest_path;
use crate::util::ext::vector::VectorExt;

mod pathfinding;

pub struct AiSystem;

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        WriteStorage<'a, Pose>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Mob>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut poses, players, mut mobs, room) = data;

        let player_pose = (&players, &poses)
            .join()
            .next()
            .map(|(_, player_pose)| player_pose.clone());

        if let Some(player_pose) = player_pose {
            for (mob, mob_pose) in (&mut mobs, &mut poses).join() {
                move_mob(mob, mob_pose, &player_pose, &room);
            }
        }
    }
}

fn move_mob(mob: &mut Mob, mob_pose: &mut Pose, player_pose: &Pose, room: &Room) {
    if can_see(mob_pose, player_pose, room) {
        mob.target = player_pose.position;
    }
    match mob.movement {
        FollowPlayer => follow_player(mob, mob_pose, room),
    }
}

fn can_see(mob_pose: &Pose, player_pose: &Pose, room: &Room) -> bool {
    let delta = mob_pose.position.direction_to(player_pose.position);
    !(1..)
        .map(|step| mob_pose.position + delta * step)
        .take_while(|pos| pos.distance(player_pose.position) > 1.)
        .any(|pos| room.cell_at(pos).blocking)
}

fn follow_player(mob: &Mob, mob_pose: &mut Pose, room: &Room) {
    mob_pose.direction = mob_pose.position.direction_to(mob.target);

    let distance = mob_pose.position.distance(mob.target);
    let min_distance = 2.;
    if distance > min_distance {
        if let Some(target) = shortest_path(mob_pose.position, mob.target, min_distance, room) {
            let direction = mob_pose.position.direction_to(target);
            mob_pose.position += mob_pose.move_in_direction(direction, mob.speed);
        }
    }
}
