use quicksilver::prelude::*;
use specs::prelude::*;

pub struct Mob {
    pub movement: MobMovement,
    pub target: Vector,
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
        }
    }
}
