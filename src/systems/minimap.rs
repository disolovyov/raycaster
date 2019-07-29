use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Transform;
use crate::config::{FOV, VH, VW};
use crate::gfx::framebuffer::{Framebuffer, RGB};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;

pub struct MinimapSystem;

const SCALE: u32 = 10;

impl<'a> System<'a> for MinimapSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Write<'a, Renderer>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, transforms, mut renderer, room) = data;

        let width = room.width() * SCALE;
        let height = room.height() * SCALE;
        let mut framebuffer = Framebuffer::new(width, height);

        for y in 0..room.height() {
            for x in 0..room.width() {
                let tile = room.get(x, y);
                framebuffer.draw_rect(x * SCALE, y * SCALE, SCALE, SCALE, room.get_color(tile));
            }
        }

        for (_, transform) in (&players, &transforms).join() {
            let fov_color = RGB(255, 0, 0);
            for ray in 0..VW {
                let angle = transform.angle - FOV / 2.0 + FOV * ray as f32 / VW as f32;

                for i in 0..20 {
                    let t = i as f32 * 0.1;
                    let x = (transform.position.x + t * angle.cos()) * SCALE as f32;
                    let y = (transform.position.y + t * angle.sin()) * SCALE as f32;
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
