use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::components::sprite::Sprite;
use crate::config::{VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;
use crate::resources::tilesets::Tilesets;
use crate::systems::player_fov::sprite::draw_sprite;
use crate::systems::player_fov::walls::draw_walls;
use crate::util::framebuffer::Framebuffer;

mod sprite;
mod walls;

pub struct PlayerFovSystem;

impl<'a> System<'a> for PlayerFovSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pose>,
        ReadStorage<'a, Sprite>,
        Write<'a, Renderer>,
        Read<'a, Room>,
        Read<'a, Tilesets>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, poses, sprites, mut renderer, room, tilesets) = data;

        if let Some((_, player_pose)) = (&players, &poses).join().next() {
            let mut framebuffer = Framebuffer::new(VW, VH);

            draw_walls(&mut framebuffer, player_pose, &room, &tilesets);
            for (sprite, sprite_pose) in (&sprites, &poses).join() {
                draw_sprite(
                    &mut framebuffer,
                    sprite,
                    sprite_pose,
                    player_pose,
                    &tilesets,
                );
            }

            renderer.add(RenderItem {
                renderable: Renderable::Framebuffer(framebuffer),
                position: Vector::ZERO,
                layer: Layer::FOV,
            });
        }
    }
}
