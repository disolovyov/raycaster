use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::mob::Mob;
use crate::components::mob::MobMovement::FollowPlayer;
use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::config::WALK_SPEED;
use crate::util::ext::vector::VectorExt;

pub struct AiSystem;

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        WriteStorage<'a, Pose>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Mob>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut poses, players, mobs) = data;

        let player_pose = (&players, &poses)
            .join()
            .next()
            .map(|(_, player_pose)| player_pose.clone());

        if let Some(player_pose) = player_pose {
            for (mob, mob_pose) in (&mobs, &mut poses).join() {
                move_mob(mob, mob_pose, &player_pose);
            }
        }
    }
}

fn move_mob(mob: &Mob, mob_pose: &mut Pose, player_pose: &Pose) {
    match mob.movement {
        FollowPlayer => follow_player(mob_pose, player_pose),
    }
}

fn follow_player(mob_pose: &mut Pose, player_pose: &Pose) {
    let angle = mob_pose.position.angle_to(player_pose.position);
    mob_pose.direction = Vector::new(angle.cos(), angle.sin());

    let distance = mob_pose.position.distance(player_pose.position);
    if distance > 2. {
        mob_pose.position += mob_pose.move_forward(WALK_SPEED / 4.);
    }
}
