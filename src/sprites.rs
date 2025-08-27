use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;

#[derive(Clone)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub texture_char: char,
    pub pos: Vector2,
    pub width: i32,
    pub height: i32,
    pub tex_data: Vec<Color>,
    pub collected: bool,
}

impl Sprite {
    pub fn from_char(
        texture_char: char,
        x: f32,
        y: f32,
        texture_cache: &TextureManager,
    ) -> Option<Self> {
        if let Some(img) = texture_cache.images.get(&texture_char) {
            // Clonamos el image data en un Vec<Color>
            let tex_data = Image::get_image_data(img);

            Some(Sprite {
                x,
                y,
                pos: Vector2::new(x, y),
                texture_char,
                width: img.width,
                height: img.height,
                tex_data: tex_data.to_vec(),
                collected: false,
            })
        } else {
            None
        }
    }
}


pub fn render_sprites(
    framebuffer: &mut Framebuffer,
    player: &Player,
    sprites: &Vec<Sprite>,
    z_buffer: &Vec<f32>,
) {
    let num_rays = framebuffer.width as usize;
    let hh = framebuffer.height as f32 / 2.0;
    let distance_to_projection_plane = 70.0;

    for sprite in sprites.iter() {
        let dx = sprite.pos.x - player.pos.x;
        let dy = sprite.pos.y - player.pos.y;
        let sprite_dist = (dx * dx + dy * dy).sqrt().max(0.0001);

        let angle_to_sprite = dy.atan2(dx);
        let mut angle_diff = angle_to_sprite - player.a;
        while angle_diff > std::f32::consts::PI {
            angle_diff -= 2.0 * std::f32::consts::PI;
        }
        while angle_diff < -std::f32::consts::PI {
            angle_diff += 2.0 * std::f32::consts::PI;
        }

        if angle_diff.abs() > player.fov / 2.0 {
            continue;
        }

        let screen_x = ((angle_diff + (player.fov / 2.0)) / player.fov) * (num_rays as f32);
        let sprite_screen_height = ((hh / sprite_dist) * distance_to_projection_plane).abs();
        let sprite_half_h = (sprite_screen_height / 2.0) as i32;
        let sprite_center_y = hh as i32;

        let tex_w = sprite.width;
        let tex_h = sprite.height;

        let left = (screen_x as i32) - sprite_half_h;
        let right = (screen_x as i32) + sprite_half_h;

        for screen_col in left.max(0)..=right.min((num_rays as i32) - 1) {
            let col_idx = screen_col as usize;
            if sprite_dist >= z_buffer[col_idx] {
                continue;
            }

            let rel = (screen_col - left) as f32 / ((right - left).max(1) as f32);
            let tex_x = (rel * tex_w as f32) as i32;

            let top = sprite_center_y - sprite_half_h;
            let bottom = sprite_center_y + sprite_half_h;

            for screen_y in top.max(0)..=bottom.min((framebuffer.height as i32) - 1) {
                let v_rel = (screen_y - top) as f32 / ((bottom - top).max(1) as f32);
                let tex_y = (v_rel * tex_h as f32) as i32;

                let tx = tex_x.clamp(0, tex_w - 1) as usize;
                let ty = tex_y.clamp(0, tex_h - 1) as usize;
                let idx = ty * (tex_w as usize) + tx;

                let color = sprite.tex_data.get(idx).copied().unwrap_or(Color::MAGENTA);

                if color.a == 0 {
                    continue;
                }

                framebuffer.set_current_color(color);
                framebuffer.set_pixel(screen_col as u32, screen_y as u32);
            }
        }
    }
}
