use quicksilver::prelude::*;

use crate::components::pose::Pose;
use crate::resources::room::RoomObject::{Door, Wall};
use crate::resources::room::{Cell, CellAt, Room};
use crate::resources::tilesets::{TilesetType, Tilesets};
use crate::util::ext::transform::TransformExt;
use crate::util::ext::vector::VectorExt;
use crate::util::framebuffer::Framebuffer;
use crate::util::rgb::RGB;

// See Lode's raycasting tutorial:
// https://lodev.org/cgtutor/raycasting.html
pub fn draw_room(
    framebuffer: &mut Framebuffer,
    zbuffer: &mut [f32],
    pose: &Pose,
    room: &Room,
    tilesets: &Tilesets,
) {
    let room_tileset = &tilesets[TilesetType::Tiles64];
    let tex_width = room_tileset.tile_width();
    let tex_height = room_tileset.tile_height();

    let fw = framebuffer.width();
    let fh = framebuffer.height() as i32;
    for ray in 0..fw {
        // Ray direction from (0, 0) to camera plane
        let ray_dir = Transform::translate(pose.direction)
            * Transform::scale_ratio(2. * ray as f32 / fw as f32 - 1.) // Screen x in (-1, 1)
            * Transform::rotate(pose.direction.angle())
            * Vector::new(0, 0.66); // Camera plane relative to (0, 1)

        let raycast_result = raycast(pose, ray_dir, room);
        zbuffer[ray as usize] = raycast_result.distance;

        // Calculate x coordinate on the wall texture
        let wall_hit = match raycast_result.hit_side {
            HitSide::X => (pose.position.y + raycast_result.distance * ray_dir.y).fract(),
            HitSide::Y => (pose.position.x + raycast_result.distance * ray_dir.x).fract(),
        };
        let mut tile_x = (wall_hit * tex_width as f32) as i32;
        if tile_x < 0 {
            tile_x += tex_width as i32;
        }

        // Render texture column
        let column_height = (fh as f32 / raycast_result.distance) as i32;
        let column_start = (fh - column_height) / 2;

        let y_start = column_start.max(0);
        let y_end = ((fh + column_height) / 2).min(fh - 1);

        // Draw wall column
        for y in y_start..y_end {
            let column_y = y - column_start;
            let tile_y = tex_height as i32 * column_y / column_height;

            let tile = match raycast_result.cell.object {
                Wall { tile } => Some(tile),
                Door { tile, .. } => Some(tile),
                _ => None,
            };
            let color = tile
                .and_then(|t| room_tileset.get_pixel(t, tile_x as u32, tile_y as u32))
                .unwrap_or_else(|| RGB::new(0, 0, 0));
            let final_color = match raycast_result.hit_side {
                HitSide::X => color,
                HitSide::Y => color.darken(),
            };
            framebuffer.draw_pixel(ray, y as u32, final_color);
        }

        let floor_texel = get_floor_texel(&raycast_result, ray_dir, wall_hit);

        // Draw floor and ceiling
        for y in y_end..fh {
            let current_dist = fh as f32 / (2 * y - fh) as f32;
            let weight = current_dist / raycast_result.distance;
            let floor_x = weight * floor_texel.x + (1. - weight) * pose.position.x;
            let floor_y = weight * floor_texel.y + (1. - weight) * pose.position.y;
            let tex_x = (floor_x * tex_width as f32) as u32 % tex_width;
            let tex_y = (floor_y * tex_height as f32) as u32 % tex_height;

            let ceiling = room_tileset
                .get_pixel(raycast_result.cell.ceiling, tex_x, tex_y)
                .unwrap_or_else(|| RGB::new(0, 0, 0));
            let floor = room_tileset
                .get_pixel(raycast_result.cell.floor, tex_x, tex_y)
                .unwrap_or_else(|| RGB::new(0, 0, 0));

            framebuffer.draw_pixel(ray, (fh - y) as u32, ceiling);
            framebuffer.draw_pixel(ray, y as u32, floor);
        }
    }
}

struct RaycastResult {
    map_pos: Vector,
    cell: Cell,
    distance: f32,
    hit_side: HitSide,
}

#[derive(PartialEq, Copy, Clone)]
enum HitSide {
    X,
    Y,
}

fn raycast(pose: &Pose, ray_dir: Vector, room: &Room) -> RaycastResult {
    // Which cell of the map we're in
    let mut map_pos = pose.position.trunc();

    // Which side was hit?
    let mut hit_side: HitSide;

    // Length of ray from one x/y-side to next x/y-side
    let delta_dist_x = (1. / ray_dir.x).abs();
    let delta_dist_y = (1. / ray_dir.y).abs();

    // Length of ray from current position to next x/y-side
    let mut side_dist_x = match ray_dir.x < 0. {
        true => (pose.position.x - map_pos.x) * delta_dist_x,
        false => (map_pos.x + 1. - pose.position.x) * delta_dist_x,
    };
    let mut side_dist_y = match ray_dir.y < 0. {
        true => (pose.position.y - map_pos.y) * delta_dist_y,
        false => (map_pos.y + 1. - pose.position.y) * delta_dist_y,
    };

    // Direction to step in
    let step = ray_dir.signum();

    // Which cell was hit?
    let mut cell: Cell;

    // Perform DDA
    loop {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_pos.x += step.x;
            hit_side = HitSide::X;
        } else {
            side_dist_y += delta_dist_y;
            map_pos.y += step.y;
            hit_side = HitSide::Y;
        }

        cell = room.cell_at(map_pos);

        match cell.object {
            Wall { .. } => {
                break;
            }
            Door { closed, .. } => {
                // Step halfway into a door
                match hit_side {
                    HitSide::X => {
                        let half_dist_x = side_dist_x - delta_dist_x / 2.;
                        let wall_hit = (pose.position.y + half_dist_x * ray_dir.y).fract();
                        if half_dist_x < side_dist_y && wall_hit < closed {
                            map_pos.x += step.x / 2.;
                            break;
                        }
                    }
                    HitSide::Y => {
                        let half_dist_y = side_dist_y - delta_dist_y / 2.;
                        let wall_hit = (pose.position.x + half_dist_y * ray_dir.x).fract();
                        if half_dist_y < side_dist_x && wall_hit < closed {
                            map_pos.y += step.y / 2.;
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Calculate distance projected on camera direction
    // No fisheye correction needed
    let distance = match hit_side {
        HitSide::X => (map_pos.x - pose.position.x + (1. - step.x) / 2.) / ray_dir.x,
        HitSide::Y => (map_pos.y - pose.position.y + (1. - step.y) / 2.) / ray_dir.y,
    };

    RaycastResult {
        cell,
        distance,
        hit_side,
        map_pos,
    }
}

fn get_floor_texel(raycast_result: &RaycastResult, ray_dir: Vector, wall_hit: f32) -> Vector {
    let hit_side = raycast_result.hit_side;
    let map_pos = raycast_result.map_pos;

    if hit_side == HitSide::X && ray_dir.x < 0. {
        Vector::new(map_pos.x + 1., map_pos.y + wall_hit)
    } else if hit_side == HitSide::X {
        Vector::new(map_pos.x, map_pos.y + wall_hit)
    } else if hit_side == HitSide::Y && ray_dir.y < 0. {
        Vector::new(map_pos.x + wall_hit, map_pos.y + 1.)
    } else {
        Vector::new(map_pos.x + wall_hit, map_pos.y)
    }
}
