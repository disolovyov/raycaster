use quicksilver::prelude::*;

use crate::components::pose::Pose;
use crate::components::sprite::{Sprite, SpriteAlign};
use crate::resources::tilesets::Tilesets;
use crate::util::framebuffer::Framebuffer;

// Max sprite width and height
const SW: f32 = 64.;
const SH: f32 = 64.;

// See Lode's raycasting tutorial:
// https://lodev.org/cgtutor/raycasting3.html
pub fn draw_sprites(
    framebuffer: &mut Framebuffer,
    zbuffer: &[f32],
    sprites: &mut [(&Sprite, &Pose)],
    player_pose: &Pose,
    tilesets: &Tilesets,
) {
    sprites.sort_by(|(_, a), (_, b)| {
        let dist_a = player_pose.position.distance(a.position);
        let dist_b = player_pose.position.distance(b.position);
        dist_b.partial_cmp(&dist_a).unwrap()
    });

    let fw = framebuffer.width() as f32;
    let fh = framebuffer.height() as f32;

    let dir = player_pose.direction;
    let camera = Transform::rotate(dir.angle()) * Vector::new(0, 0.66); // Camera plane relative to (0, 1)

    for (sprite, sprite_pose) in sprites {
        // Transform sprite with the inverse camera matrix
        let dt = sprite_pose.position - player_pose.position;
        let inv_det = 1. / (camera.x * dir.y - dir.x * camera.y);
        let tx = inv_det * (dir.y * dt.x - dir.x * dt.y);
        let ty = inv_det * (-camera.y * dt.x + camera.x * dt.y);

        if ty <= 0. {
            continue;
        }

        let tileset = &tilesets[sprite.tileset()];
        let tile = sprite.tileset_id();
        let tw = tileset.tile_width() as f32;
        let th = tileset.tile_height() as f32;

        let sprite_x = fw / 2. * (1. + tx / ty);
        let max_sprite_h = (fh / ty).abs();
        let sprite_h = max_sprite_h * (th / SH);
        let start_dy = match sprite.align() {
            SpriteAlign::Top => (sprite_h - max_sprite_h) / 2.,
            SpriteAlign::Bottom => (max_sprite_h - sprite_h) / 2.,
        };

        let start_y = fh / 2. - sprite_h / 2. + start_dy;
        let from_y = start_y.max(0.) as i32;
        let to_y = (start_y + sprite_h).min(fh - 1.) as i32;

        let sprite_w = max_sprite_h * (tw / SW);
        let start_x = sprite_x - sprite_w / 2.;
        let from_x = start_x.max(0.) as i32;
        let to_x = (start_x + sprite_w).min(fw - 1.) as i32;

        let fw = framebuffer.width() as i32;
        for x in from_x..to_x {
            if x < 0 || x >= fw || ty >= zbuffer[x as usize] {
                continue;
            }
            let tile_x = ((x as f32 - start_x) / sprite_w * tw) as u32;
            for y in from_y..to_y {
                let tile_y = ((y as f32 - start_y) / sprite_h * th) as u32;
                if let Some(color) = tileset.get_pixel(tile, tile_x, tile_y) {
                    framebuffer.draw_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}
