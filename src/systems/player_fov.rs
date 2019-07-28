use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Transform;
use crate::config::{FOV, VH, VW};
use crate::gfx::framebuffer::Framebuffer;
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;

pub struct PlayerFovSystem;

impl<'a> System<'a> for PlayerFovSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Write<'a, Renderer>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, transforms, mut renderer, room) = data;

        let mut framebuffer = Framebuffer::new(VW, VH);

        for (_, transform) in (&players, &transforms).join() {
            for ray in 0..VW {
                let angle = transform.angle - FOV / 2.0 + FOV * ray as f32 / VW as f32;

                for i in 0..2000 {
                    let t = i as f32 * 0.01;
                    let cx = transform.position.x + t * angle.cos();
                    let cy = transform.position.y + t * angle.sin();

                    let cell = room.get(cx as u32, cy as u32);
                    if cell != 0 {
                        let mut column_height =
                            (VW as f32 / (t * (angle - transform.angle).cos())) as u32;
                        if column_height > VH {
                            column_height = VH;
                        }
                        framebuffer.draw_rect(
                            ray,
                            (VH - column_height) / 2,
                            1,
                            column_height,
                            room.get_color(cell),
                        );
                        break;
                    }
                }
            }
        }

        renderer.add(RenderItem {
            renderable: Renderable::Framebuffer(framebuffer),
            position: Vector::ZERO,
            layer: Layer::FOV,
        })
    }
}
