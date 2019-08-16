use quicksilver::prelude::*;
use specs::prelude::*;

use crate::config::WALK_SPEED;

pub struct Mob {
    pub movement: MobMovement,
    pub target: Vector,
    pub speed: f32,
}

pub enum MobMovement {
    FollowPlayer,
}

impl Component for Mob {
    type Storage = VecStorage<Self>;
}

impl Mob {
    pub fn new(target: Vector) -> Mob {
        Mob {
            movement: MobMovement::FollowPlayer,
            target,
            speed: WALK_SPEED / 4.,
        }
    }
}
