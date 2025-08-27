// player.rs

use raylib::prelude::*;
use std::f32::consts::PI;


pub struct Player {
    pub pos: Vector2,
    pub a: f32,      // angle (direction the player is facing)
    pub fov: f32,    // field of view
    pub plane: Vector2, // camera plane vector
}

impl Player {
    pub fn new(pos: Vector2, a: f32, fov: f32) -> Self {
        let plane = Self::compute_plane(a, fov);
        Self { pos, a, fov, plane }
    }

    fn compute_plane(a: f32, fov: f32) -> Vector2 {
        // half FOV tangent gives camera plane scale
        let plane_x = a.sin() * (fov / 2.0).tan();
        let plane_y = -a.cos() * (fov / 2.0).tan();
        Vector2::new(plane_x, plane_y)
    }

    pub fn update_plane(&mut self) {
        self.plane = Self::compute_plane(self.a, self.fov);
    }
}

pub fn process_events(player: &mut Player, rl: &RaylibHandle) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;
    const MOUSE_SENSITIVITY: f32 = 0.005;

    // Rotation
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a -= ROTATION_SPEED;
    }

    // Movement
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        player.pos.x -= MOVE_SPEED * player.a.cos();
        player.pos.y -= MOVE_SPEED * player.a.sin();
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        player.pos.x += MOVE_SPEED * player.a.cos();
        player.pos.y += MOVE_SPEED * player.a.sin();
    }

    // Mouse look
    let mouse_delta = rl.get_mouse_delta();
    player.a -= mouse_delta.x * MOUSE_SENSITIVITY;

    // Recompute camera plane after any rotation change
    player.update_plane();
}
