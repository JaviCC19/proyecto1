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
    const GAMEPAD_SENSITIVITY: f32 = 0.05; // para rotación analógica
    const DEADZONE: f32 = 0.2; // para evitar drift del joystick

    // --- Controles por teclado ---
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a -= ROTATION_SPEED;
    }

    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        player.pos.x -= MOVE_SPEED * player.a.cos();
        player.pos.y -= MOVE_SPEED * player.a.sin();
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        player.pos.x += MOVE_SPEED * player.a.cos();
        player.pos.y += MOVE_SPEED * player.a.sin();
    }

    // --- Controles por mouse (rotación) ---
    let mouse_delta = rl.get_mouse_delta();
    player.a -= mouse_delta.x * MOUSE_SENSITIVITY;

    // --- Controles por gamepad ---
    let gamepad_index = 0;
    if rl.is_gamepad_available(gamepad_index) {
        let left_x = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
        let left_y = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
        let right_x = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
        let _right_y = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_RIGHT_Y);

        // Movimiento con el stick izquierdo
        if left_y.abs() > DEADZONE {
            player.pos.x += -left_y * MOVE_SPEED * player.a.cos();
            player.pos.y += -left_y * MOVE_SPEED * player.a.sin();
        }
        if left_x.abs() > DEADZONE {
            // Strafe (moverse de lado)
            player.pos.x += left_x * MOVE_SPEED * player.a.sin();
            player.pos.y += -left_x * MOVE_SPEED * player.a.cos();
        }

        // Rotación con el stick derecho (eje X)
        if right_x.abs() > DEADZONE {
            player.a -= right_x * GAMEPAD_SENSITIVITY;
        }
    }

    // --- Actualizar plano de cámara ---
    player.update_plane();
}
