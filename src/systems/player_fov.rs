use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::components::sprite::Sprite;
use crate::config::{VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;
use crate::resources::tilesets::Tilesets;
use crate::systems::player_fov::room::draw_room;
use crate::systems::player_fov::sprite::draw_sprites;
use crate::util::framebuffer::Framebuffer;

mod room;
mod sprite;

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
            let mut zbuffer: Vec<f32> = vec![0.; VW as usize];

            draw_room(
                &mut framebuffer,
                &mut zbuffer,
                player_pose,
                &room,
                &tilesets,
            );

            let mut sprites: Vec<(&Sprite, &Pose)> = (&sprites, &poses).join().collect();

            draw_sprites(
                &mut framebuffer,
                &zbuffer,
                &mut sprites,
                player_pose,
                &tilesets,
            );

            renderer.add(RenderItem {
                renderable: Renderable::Framebuffer(framebuffer),
                position: Vector::ZERO,
                layer: Layer::FOV,
            });
        }
    }
}
