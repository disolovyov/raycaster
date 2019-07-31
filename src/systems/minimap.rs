use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;
use crate::config::{FOV, VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;
use crate::util::framebuffer::{Framebuffer, RGB};

pub struct MinimapSystem;

const SCALE: u32 = 10;

impl<'a> System<'a> for MinimapSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pose>,
        Write<'a, Renderer>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, poses, mut renderer, room) = data;

        let width = room.width() * SCALE;
        let height = room.height() * SCALE;
        let mut framebuffer = Framebuffer::new(width, height);

        for y in 0..room.height() {
            for x in 0..room.width() {
                let tile = room.get(x, y);
                framebuffer.draw_rect(x * SCALE, y * SCALE, SCALE, SCALE, room.get_color(tile));
            }
        }

        if let Some((_, pose)) = (&players, &poses).join().next() {
            let fov_color = RGB(255, 255, 255);
            let pose_angle = pose.direction.angle().to_radians();
            for ray in [0, VW - 1].iter() {
                let angle = pose_angle - FOV / 2.0 + FOV * *ray as f32 / VW as f32;

                for i in 0..200 {
                    let t = i as f32 * 0.1;
                    let x = (pose.position.x + t * angle.cos()) * SCALE as f32;
                    let y = (pose.position.y + t * angle.sin()) * SCALE as f32;
                    if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
                        framebuffer.draw_pixel(x as u32, y as u32, fov_color);
                    }
                }
            }
        }

        renderer.add(RenderItem {
            renderable: Renderable::Framebuffer(framebuffer),
            position: Vector::new(VW - width - 10, VH - height - 10),
            layer: Layer::UI,
        })
    }
}
