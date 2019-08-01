use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::transform::Pose;
use crate::config::{VH, VW};
use crate::resources::renderer::{Layer, RenderItem, Renderable, Renderer};
use crate::resources::room::{Room, TILE_SIZE};
use crate::util::framebuffer::Framebuffer;
use crate::util::transform::TransformExt;

pub struct PlayerFovSystem;

impl<'a> System<'a> for PlayerFovSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pose>,
        Write<'a, Renderer>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, poses, mut renderer, room) = data;

        if let Some((_, pose)) = (&players, &poses).join().next() {
            let framebuffer = PlayerFovSystem::render(pose, &room);
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
    fn render(pose: &Pose, room: &Room) -> Framebuffer {
        let mut framebuffer = Framebuffer::new(VW, VH);
        for ray in 0..VW {
            // Ray direction from (0, 0) to camera plane
            let ray_dir = Transform::translate(pose.direction)
                * Transform::scale_ratio(2.0 * ray as f32 / VW as f32 - 1.0) // Screen x in (-1, 1)
                * Transform::rotate(pose.direction.angle())
                * Vector::new(0, -0.66); // Camera plane relative to (0, 1)

            // Which cell of the map we're in
            let mut map_x = pose.position.x.trunc();
            let mut map_y = pose.position.y.trunc();

            // Length of ray from one x/y-side to next x/y-side
            let delta_dist_x = (1.0 / ray_dir.x).abs();
            let delta_dist_y = (1.0 / ray_dir.y).abs();

            // Direction to step in
            let step_x = ray_dir.x.signum();
            let step_y = ray_dir.y.signum();

            // Length of ray from current position to next x/y-side
            let mut side_dist_x = match ray_dir.x < 0.0 {
                true => (pose.position.x - map_x) * delta_dist_x,
                false => (map_x + 1.0 - pose.position.x) * delta_dist_x,
            };
            let mut side_dist_y = match ray_dir.y < 0.0 {
                true => (pose.position.y - map_y) * delta_dist_y,
                false => (map_y + 1.0 - pose.position.y) * delta_dist_y,
            };

            let mut y_side: bool; // Which wall was hit?

            // Perform DDA
            loop {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    y_side = false;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    y_side = true;
                }
                if room.is_solid(map_x as u32, map_y as u32) {
                    break;
                }
            }

            // Calculate distance projected on camera direction
            // No fisheye correction needed
            let perp_wall_dist = match y_side {
                false => (map_x - pose.position.x + (1.0 - step_x) / 2.0) / ray_dir.x,
                true => (map_y - pose.position.y + (1.0 - step_y) / 2.0) / ray_dir.y,
            };
            if perp_wall_dist <= 0.0 {
                continue;
            }

            // Calculate texture column span
            let column_height = (VH as f32 / perp_wall_dist) as u32;
            let column_vh = VH as i32;
            let column_start = ((column_vh - column_height as i32) / 2).max(0) as u32;
            let column_end = ((column_vh + column_height as i32) / 2).min(column_vh - 1) as u32;

            // Calculate x coordinate on the wall texture
            let wall_hit = match y_side {
                false => (pose.position.y + perp_wall_dist * ray_dir.y).fract(),
                true => (pose.position.x + perp_wall_dist * ray_dir.x).fract(),
            };
            let mut tile_x = (wall_hit * TILE_SIZE as f32) as i32;
            if tile_x < 0 {
                tile_x += TILE_SIZE as i32;
            }

            // Render texture column
            let tile = room.get(map_x as u32, map_y as u32);
            for y in column_start..column_end {
                let tile_y = TILE_SIZE as u32 * (y - column_start) / column_height;
                let pixel = room.get_texture_pixel(tile, tile_x as usize, tile_y as usize);
                let color = match y_side {
                    false => pixel,
                    true => pixel.darken(),
                };
                framebuffer.draw_pixel(ray, y, color);
            }
        }
        framebuffer
    }
}
