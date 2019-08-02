use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;
use crate::config::{VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::Room;
use crate::resources::tilesets::Tilesets;
use crate::util::framebuffer::{Framebuffer, RGB};
use crate::util::tileset::Tileset;
use crate::util::transform::TransformExt;
use crate::util::vector::VectorExt;

pub struct PlayerFovSystem;

impl<'a> System<'a> for PlayerFovSystem {
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

        if let Some((_, pose)) = (&players, &poses).join().next() {
            let framebuffer = PlayerFovSystem::render(pose, &room, walls);
            renderer.add(RenderItem {
                renderable: Renderable::Framebuffer(framebuffer),
                position: Vector::ZERO,
                layer: Layer::FOV,
            })
        }
    }
}

impl PlayerFovSystem {
    // See Lode's raycasting tutorial:
    // https://lodev.org/cgtutor/raycasting.html
    fn render(pose: &Pose, room: &Room, walls: &Tileset) -> Framebuffer {
        let mut framebuffer = Framebuffer::new(VW, VH);
        for ray in 0..VW {
            // Ray direction from (0, 0) to camera plane
            let ray_dir = Transform::translate(pose.direction)
                * Transform::scale_ratio(2. * ray as f32 / VW as f32 - 1.) // Screen x in (-1, 1)
                * Transform::rotate(pose.direction.angle())
                * Vector::new(0, 0.66); // Camera plane relative to (0, 1)

            // Which cell of the map we're in
            let mut map_pos = pose.position.trunc();

            // Length of ray from one x/y-side to next x/y-side
            let delta_dist_x = (1. / ray_dir.x).abs();
            let delta_dist_y = (1. / ray_dir.y).abs();

            // Direction to step in
            let step_x = ray_dir.x.signum();
            let step_y = ray_dir.y.signum();

            // Length of ray from current position to next x/y-side
            let mut side_dist_x = match ray_dir.x < 0. {
                true => (pose.position.x - map_pos.x) * delta_dist_x,
                false => (map_pos.x + 1. - pose.position.x) * delta_dist_x,
            };
            let mut side_dist_y = match ray_dir.y < 0. {
                true => (pose.position.y - map_pos.y) * delta_dist_y,
                false => (map_pos.y + 1. - pose.position.y) * delta_dist_y,
            };

            let mut y_side: bool; // Which wall was hit?

            // Perform DDA
            loop {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_pos.x += step_x;
                    y_side = false;
                } else {
                    side_dist_y += delta_dist_y;
                    map_pos.y += step_y;
                    y_side = true;
                }
                if room.is_solid(&map_pos) {
                    break;
                }
            }

            // Calculate distance projected on camera direction
            // No fisheye correction needed
            let perp_wall_dist = match y_side {
                false => (map_pos.x - pose.position.x + (1. - step_x) / 2.) / ray_dir.x,
                true => (map_pos.y - pose.position.y + (1. - step_y) / 2.) / ray_dir.y,
            };
            if perp_wall_dist <= 0.1 {
                continue;
            }

            // Calculate x coordinate on the wall texture
            let wall_hit = match y_side {
                false => (pose.position.y + perp_wall_dist * ray_dir.y).fract(),
                true => (pose.position.x + perp_wall_dist * ray_dir.x).fract(),
            };
            let mut tile_x = (wall_hit * walls.tile_width() as f32) as i32;
            if tile_x < 0 {
                tile_x += walls.tile_width() as i32;
            }

            // Render texture column
            let tile = room.get_tile(&map_pos);
            let vh = VH as i32;
            let column_height = (VH as f32 / perp_wall_dist) as i32;
            let column_start = (vh - column_height) / 2;

            let y_start = column_start.max(0);
            let y_end = ((vh + column_height) / 2).min(vh - 1);

            // Draw column
            for y in 0..y_start {
                framebuffer.draw_pixel(ray, y as u32, RGB(56, 56, 56));
            }
            for y in y_start..y_end {
                let column_y = y - column_start;
                let tile_y = walls.tile_height() as i32 * column_y / column_height;
                let pixel = walls.get_pixel(tile, tile_x as u32, tile_y as u32);
                let color = match y_side {
                    false => pixel,
                    true => pixel.darken(),
                };
                framebuffer.draw_pixel(ray, y as u32, color);
            }
            for y in y_end..vh {
                framebuffer.draw_pixel(ray, y as u32, RGB(113, 113, 113));
            }
        }
        framebuffer
    }
}
