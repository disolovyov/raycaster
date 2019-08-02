use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;
use crate::config::{VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;
use crate::resources::tilesets::Tilesets;
use crate::util::framebuffer::{Framebuffer, RGB};

pub struct MinimapSystem;

const SCALE: u32 = 8;

impl<'a> System<'a> for MinimapSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pose>,
        Write<'a, Renderer>,
        Read<'a, Room>,
        Read<'a, Tilesets>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, poses, mut renderer, room, tilesets) = data;

        let walls = tilesets.walls();
        let width = room.width() * SCALE;
        let height = room.height() * SCALE;
        let mut framebuffer = Framebuffer::new(width, height);

        for y in 0..room.height() {
            for x in 0..room.width() {
                let color = match room.get_tile_xy(x, y) {
                    0 => RGB(0, 0, 0),
                    tile => walls.get_pixel(tile, 1, 1),
                };
                framebuffer.draw_rect(x * SCALE, y * SCALE, SCALE, SCALE, color);
            }
        }

        if let Some((_, pose)) = (&players, &poses).join().next() {
            draw_player(&mut framebuffer, pose)
        }

        renderer.add(RenderItem {
            renderable: Renderable::Framebuffer(framebuffer),
            position: Vector::new(VW - width - 10, VH - height - 10),
            layer: Layer::UI,
        })
    }
}

fn draw_player(framebuffer: &mut Framebuffer, pose: &Pose) {
    let x = (pose.position.x * SCALE as f32) as u32;
    let y = (pose.position.y * SCALE as f32) as u32;
    let color = RGB(255, 255, 255);
    framebuffer.draw_rect(x - 1, y - 1, 3, 3, color);
}
