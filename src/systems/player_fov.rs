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

        if let Some((_, transform)) = (&players, &transforms).join().next() {
            let framebuffer = PlayerFovSystem::render(transform, &room);
            renderer.add(RenderItem {
                renderable: Renderable::Framebuffer(framebuffer),
                position: Vector::ZERO,
                layer: Layer::FOV,
            })
        }
    }
}

impl PlayerFovSystem {
    fn render(transform: &Transform, room: &Room) -> Framebuffer {
        let mut framebuffer = Framebuffer::new(VW, VH);
        for ray in 0..VW {
            let angle = transform.angle - FOV / 2.0 + FOV * ray as f32 / VW as f32;

            for i in 0..2000 {
                let t = i as f32 * 0.01;
                let cx: f32 = transform.position.x + t * angle.cos();
                let cy: f32 = transform.position.y + t * angle.sin();

                let tile = room.get(cx as u32, cy as u32);
                if tile == 0 {
                    continue;
                }

                let column_height = (VW as f32 / (t * (angle - transform.angle).cos())) as i32;

                let texture_x = PlayerFovSystem::get_texture_x(cx, cy);
                let texture_column =
                    room.get_texture_column(tile, texture_x as usize, column_height as usize);

                let column_start = VH as i32 / 2 - column_height / 2;
                for column_y in 0..column_height {
                    let y = column_start + column_y;
                    if y >= 0 && y < VH as i32 {
                        framebuffer.draw_pixel(ray, y as u32, texture_column[column_y as usize])
                    }
                }

                break;
            }
        }
        framebuffer
    }

    fn get_texture_x(cx: f32, cy: f32) -> i32 {
        let hit_x = cx - (cx + 0.5).floor();
        let hit_y = cy - (cy + 0.5).floor();
        let mut texture_x = match hit_y.abs() > hit_x.abs() {
            true => (hit_y * TILE_SIZE as f32) as i32,
            false => (hit_x * TILE_SIZE as f32) as i32,
        };
        if texture_x < 0 {
            texture_x += TILE_SIZE as i32;
        }
        texture_x
    }
}
