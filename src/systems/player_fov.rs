use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Transform;
use crate::config::{FOV, VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::{Room, TILE_SIZE};
use crate::util::framebuffer::Framebuffer;

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
                    let cx: f32 = transform.position.x + t * angle.cos();
                    let cy: f32 = transform.position.y + t * angle.sin();

                    let tile = room.get(cx as u32, cy as u32);
                    if tile != 0 {
                        let mut column_height =
                            (VW as f32 / (t * (angle - transform.angle).cos())) as u32;
                        if column_height > VH {
                            column_height = VH;
                        }

                        let hit_x = cx - (cx + 0.5).floor();
                        let hit_y = cy - (cy + 0.5).floor();
                        let mut texture_x = match hit_y.abs() > hit_x.abs() {
                            true => (hit_y * TILE_SIZE as f32) as i32,
                            false => (hit_x * TILE_SIZE as f32) as i32,
                        };
                        if texture_x < 0 {
                            texture_x += TILE_SIZE as i32;
                        }

                        let texture_column = room.get_texture_column(
                            tile,
                            texture_x as usize,
                            column_height as usize,
                        );

                        let x = ray;
                        let column_start = VH / 2 - column_height / 2;
                        for column_y in 0..column_height {
                            let y = column_start + column_y;
                            if y < VH {
                                framebuffer.draw_pixel(x, y, texture_column[column_y as usize])
                            }
                        }

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
