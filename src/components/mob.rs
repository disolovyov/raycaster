use specs::prelude::*;

pub struct Mob {
    pub movement: MobMovement,
}

pub enum MobMovement {
    FollowPlayer,
}

impl Default for Mob {
    fn default() -> Self {
        Mob {
            movement: MobMovement::FollowPlayer,
        }
    }
}

impl Component for Mob {
    type Storage = VecStorage<Self>;
}
