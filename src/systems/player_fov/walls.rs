use quicksilver::prelude::*;

use crate::components::pose::Pose;
use crate::resources::room::Room;
use crate::resources::tilesets::{TilesetType, Tilesets};
use crate::util::ext::transform::TransformExt;
use crate::util::ext::vector::VectorExt;
use crate::util::framebuffer::Framebuffer;
use crate::util::rgb::RGB;

// See Lode's raycasting tutorial:
// https://lodev.org/cgtutor/raycasting.html
pub fn draw_walls(
    framebuffer: &mut Framebuffer,
    zbuffer: &mut [f32],
    pose: &Pose,
    room: &Room,
    tilesets: &Tilesets,
) {
    let mut fov_renderer = FovRenderer {
        framebuffer,
        zbuffer,
        pose,
        room,
        tilesets,
    };
    fov_renderer.render();
}

struct FovRenderer<'a> {
    framebuffer: &'a mut Framebuffer,
    zbuffer: &'a mut [f32],
    pose: &'a Pose,
    room: &'a Room,
    tilesets: &'a Tilesets,
}

enum WallSide {
    X,
    Y,
}

struct RaycastResult {
    map_pos: Vector,
    distance: f32,
    hit_side: WallSide,
}

impl<'a> FovRenderer<'a> {
    fn render(&mut self) {
        let walls = &self.tilesets[TilesetType::Walls];

        let fw = self.framebuffer.width();
        let fh = self.framebuffer.height() as i32;
        for ray in 0..fw {
            // Ray direction from (0, 0) to camera plane
            let ray_dir = Transform::translate(self.pose.direction)
                * Transform::scale_ratio(2. * ray as f32 / fw as f32 - 1.) // Screen x in (-1, 1)
                * Transform::rotate(self.pose.direction.angle())
                * Vector::new(0, 0.66); // Camera plane relative to (0, 1)

            let raycast_result = self.raycast(ray_dir);
            self.zbuffer[ray as usize] = raycast_result.distance;

            // Calculate x coordinate on the wall texture
            let wall_hit = match raycast_result.hit_side {
                WallSide::X => (self.pose.position.y + raycast_result.distance * ray_dir.y).fract(),
                WallSide::Y => (self.pose.position.x + raycast_result.distance * ray_dir.x).fract(),
            };
            let mut tile_x = (wall_hit * walls.tile_width() as f32) as i32;
            if tile_x < 0 {
                tile_x += walls.tile_width() as i32;
            }

            // Render texture column
            let tile = self.room.get_tile(raycast_result.map_pos);
            let column_height = (fh as f32 / raycast_result.distance) as i32;
            let column_start = (fh - column_height) / 2;

            let y_start = column_start.max(0);
            let y_end = ((fh + column_height) / 2).min(fh - 1);

            // Draw column
            for y in 0..y_start {
                self.framebuffer
                    .draw_pixel(ray, y as u32, RGB::new(56, 56, 56));
            }
            for y in y_start..y_end {
                let column_y = y - column_start;
                let tile_y = walls.tile_height() as i32 * column_y / column_height;
                let pixel = walls
                    .get_pixel(tile, tile_x as u32, tile_y as u32)
                    .unwrap_or_else(|| RGB::new(0, 0, 0));
                let color = match raycast_result.hit_side {
                    WallSide::X => pixel,
                    WallSide::Y => pixel.darken(),
                };
                self.framebuffer.draw_pixel(ray, y as u32, color);
            }
            for y in y_end..fh {
                self.framebuffer
                    .draw_pixel(ray, y as u32, RGB::new(113, 113, 113));
            }
        }
    }

    fn raycast(&self, ray_dir: Vector) -> RaycastResult {
        // Which cell of the map we're in
        let mut map_pos = self.pose.position.trunc();

        // Which wall was hit?
        let mut hit_side: WallSide;

        // Length of ray from one x/y-side to next x/y-side
        let delta_dist_x = (1. / ray_dir.x).abs();
        let delta_dist_y = (1. / ray_dir.y).abs();

        // Length of ray from current position to next x/y-side
        let mut side_dist_x = match ray_dir.x < 0. {
            true => (self.pose.position.x - map_pos.x) * delta_dist_x,
            false => (map_pos.x + 1. - self.pose.position.x) * delta_dist_x,
        };
        let mut side_dist_y = match ray_dir.y < 0. {
            true => (self.pose.position.y - map_pos.y) * delta_dist_y,
            false => (map_pos.y + 1. - self.pose.position.y) * delta_dist_y,
        };

        // Direction to step in
        let step = ray_dir.signum();

        // Perform DDA
        loop {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_pos.x += step.x;
                hit_side = WallSide::X;
            } else {
                side_dist_y += delta_dist_y;
                map_pos.y += step.y;
                hit_side = WallSide::Y;
            }
            if self.room.get_tile(map_pos) != 0 {
                break;
            }
        }
        // Calculate distance projected on camera direction
        // No fisheye correction needed
        let wall_distance = match hit_side {
            WallSide::X => (map_pos.x - self.pose.position.x + (1. - step.x) / 2.) / ray_dir.x,
            WallSide::Y => (map_pos.y - self.pose.position.y + (1. - step.y) / 2.) / ray_dir.y,
        };

        RaycastResult {
            distance: wall_distance,
            hit_side,
            map_pos,
        }
    }
}
