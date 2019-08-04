use crate::components::pose::Pose;
use crate::components::sprite::Sprite;
use crate::resources::tilesets::Tilesets;
use crate::util::framebuffer::Framebuffer;

// See Lode's raycasting tutorial:
// https://lodev.org/cgtutor/raycasting3.html
pub fn draw_sprite(
    framebuffer: &mut Framebuffer,
    sprite: &Sprite,
    sprite_pose: &Pose,
    player_pose: &Pose,
    tilesets: &Tilesets,
) {
    // TODO
}
