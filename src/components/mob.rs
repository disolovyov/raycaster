use specs::{Component, NullStorage};

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
    type Storage = NullStorage<Self>;
}
