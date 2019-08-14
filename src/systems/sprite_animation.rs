use specs::prelude::*;

use crate::components::sprite::Sprite;
use crate::resources::animation::Animation;

pub struct SpriteAnimationSystem;

impl<'a> System<'a> for SpriteAnimationSystem {
    type SystemData = (WriteStorage<'a, Sprite>, Read<'a, Animation>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprites, animation) = data;

        for sprite in (&mut sprites).join() {
            sprite.animate(&animation);
        }
    }
}
