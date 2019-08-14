use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::mob::Mob;
use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::components::prop::Prop;
use crate::config::{VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::RoomObject::{Door, Wall};
use crate::resources::room::{CellAt, Room};
use crate::resources::tilesets::{TilesetType, Tilesets};
use crate::util::framebuffer::Framebuffer;
use crate::util::rgb::RGB;

pub struct MinimapSystem;

const SCALE: u32 = 6;

impl<'a> System<'a> for MinimapSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Mob>,
        ReadStorage<'a, Prop>,
        ReadStorage<'a, Pose>,
        Write<'a, Renderer>,
        Read<'a, Room>,
        Read<'a, Tilesets>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mobs, props, poses, mut renderer, room, tilesets) = data;

        let room_tileset = &tilesets[TilesetType::Tiles64];
        let width = room.width() * SCALE;
        let height = room.height() * SCALE;
        let mut framebuffer = Framebuffer::new(width, height);

        for y in 0..room.height() {
            for x in 0..room.width() {
                let cell = room.cell_at((x, y));
                let tile = match cell.object {
                    Wall { tile } => Some(tile),
                    Door { tile, .. } => Some(tile),
                    _ => None,
                };
                let color = tile
                    .and_then(|t| room_tileset.get_pixel(t, 1, 1))
                    .unwrap_or_else(|| RGB::new(0, 0, 0));
                framebuffer.draw_rect(x * SCALE, y * SCALE, SCALE, SCALE, color);
            }
        }

        if let Some((_, pose)) = (&players, &poses).join().next() {
            let player_color = RGB::new(255, 255, 255);
            draw_position(&mut framebuffer, pose, player_color);
            draw_rotation(&mut framebuffer, pose, player_color);
        }

        let mob_color = RGB::new(255, 0, 0);
        for (_, pose) in (&mobs, &poses).join() {
            draw_position(&mut framebuffer, pose, mob_color);
            draw_rotation(&mut framebuffer, pose, mob_color);
        }

        let prop_color = RGB::new(192, 192, 192);
        let blocking_props = (&props, &poses)
            .join()
            .filter_map(|(_, pose)| Some(pose).filter(|_| room.cell_at(pose.position).blocking));
        for pose in blocking_props {
            draw_position(&mut framebuffer, pose, prop_color);
        }

        renderer.add(RenderItem {
            renderable: Renderable::Framebuffer(framebuffer),
            position: Vector::new(VW - width - 10, VH - height - 10),
            layer: Layer::UI,
        })
    }
}

fn draw_position(framebuffer: &mut Framebuffer, pose: &Pose, color: RGB) {
    let scale = SCALE as f32;
    let x = pose.position.x * scale;
    let y = pose.position.y * scale;
    framebuffer.draw_rect(x as u32 - 1, y as u32 - 1, 3, 3, color);
}

fn draw_rotation(framebuffer: &mut Framebuffer, pose: &Pose, color: RGB) {
    let scale = SCALE as f32;
    let dir_x = (pose.position.x + pose.direction.x) * scale;
    let dir_y = (pose.position.y + pose.direction.y) * scale;
    framebuffer.draw_pixel(dir_x as u32, dir_y as u32, color);
}
