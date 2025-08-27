use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;
use raylib::prelude::*;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub texture_char: char,
}

const TRANSPARENT_COLOR: Color = Color::new(152, 0, 136, 255); 
// Color clave usado como "transparente" (magenta típico en sprites sin canal alfa)

pub fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Sprite,                 // usamos tu Sprite como "enemigo"
    texture_manager: &TextureManager,
) {
    // ------------------------------------------------------
    // 1. Calcular posición relativa del enemigo al jugador
    // ------------------------------------------------------
    let dx = enemy.x - player.pos.x;
    let dy = enemy.y - player.pos.y;

    // Ángulo absoluto hacia el enemigo
    let angle_to_enemy = dy.atan2(dx);

    // Diferencia angular entre la dirección del jugador y el enemigo
    let mut angle_diff = angle_to_enemy - player.a;

    // Normalizamos ángulo a rango [-PI, PI]
    if angle_diff > std::f32::consts::PI {
        angle_diff -= 2.0 * std::f32::consts::PI;
    }
    if angle_diff < -std::f32::consts::PI {
        angle_diff += 2.0 * std::f32::consts::PI;
    }

    // ------------------------------------------------------
    // 2. Comprobar si está dentro del FOV
    // ------------------------------------------------------
    if angle_diff.abs() > player.fov / 2.0 {
        return; // enemigo fuera de la vista, no dibujar
    }

    // ------------------------------------------------------
    // 3. Calcular distancia jugador-enemigo
    // ------------------------------------------------------
    let distance = (dx * dx + dy * dy).sqrt();
    if distance <= 0.1 {
        return; // demasiado cerca o encima del jugador
    }

    // ------------------------------------------------------
    // 4. Calcular tamaño del sprite en pantalla
    //    (escala inversamente proporcional a la distancia)
    // ------------------------------------------------------
    let sprite_size = (framebuffer.height as f32 / distance) as usize;

    // ------------------------------------------------------
    // 5. Calcular posición horizontal (centrada en pantalla)
    // ------------------------------------------------------
    let screen_center_x = framebuffer.width as f32 / 2.0;
    let screen_x = screen_center_x + angle_diff * framebuffer.width as f32 / player.fov;

    // Coordenadas de la esquina superior izquierda del sprite
    let start_x = (screen_x as isize - (sprite_size as isize / 2))
        .max(0) as usize;
    let start_y = (framebuffer.height as isize / 2 - sprite_size as isize / 2)
        .max(0) as usize;

    // ------------------------------------------------------
    // 6. Calcular límites de dibujo (para recorte con pantalla)
    // ------------------------------------------------------
    let end_x = (start_x + sprite_size).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size).min(framebuffer.height as usize);

    // ------------------------------------------------------
    // 7. Obtener dimensiones de la textura real
    // ------------------------------------------------------
    let (tex_w, tex_h) = if let Some(image) = texture_manager.images.get(&enemy.texture_char) {
        (image.width as usize, image.height as usize)
    } else {
        return; // si no existe la textura, no dibujar
    };

    // ------------------------------------------------------
    // 8. Dibujo del sprite: mapear coordenadas pantalla → textura
    // ------------------------------------------------------
    for x in start_x..end_x {
        for y in start_y..end_y {
            // Calcular coordenadas de textura proporcionales
            let tx = ((x - start_x) * tex_w / sprite_size) as u32;
            let ty = ((y - start_y) * tex_h / sprite_size) as u32;

            // Obtener color del pixel desde la textura
            let color = texture_manager.get_pixel_color(enemy.texture_char, tx, ty);

            // Saltar píxeles transparentes (colorkey)
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}
