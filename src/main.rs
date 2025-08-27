#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod textures;
pub mod sprites;

use line::line;
use maze::{Maze, load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

use crate::maze::is_wall;
use crate::sprites::Sprite;
use crate::sprites::render_sprites;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

use crate::textures::TextureManager;

fn cell_to_texture_color(
    texture_cache: &TextureManager,
    cell: char,
    tx: u32,
    ty: u32,
) -> Color {
    texture_cache.get_pixel_color(cell, tx, ty)
}

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::BLUEVIOLET,
        '-' => Color::VIOLET,
        '|' => Color::VIOLET,
        'g' => Color::GREEN,
        'p' => Color::RED, // jugador en minimapa
        _ => Color::WHITE,
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
    texture_cache: &TextureManager,
) {
    if cell == ' ' {
        return; // no dibujar nada si es espacio
    }

    if let Some(image) = texture_cache.images.get(&cell) {
        // pintar textura reducida (para el minimapa)
        let tex_w = image.width as usize;
        let tex_h = image.height as usize;

        for x in 0..block_size {
            for y in 0..block_size {
                let tx = (x * tex_w) / block_size;
                let ty = (y * tex_h) / block_size;
                let color = texture_cache.get_pixel_color(cell, tx as u32, ty as u32);
                framebuffer.set_current_color(color);
                framebuffer.set_pixel((xo + x) as u32, (yo + y) as u32);
            }
        }
    } else {
        // fallback solo si no hay textura
        framebuffer.set_current_color(cell_to_color(cell));
        for x in xo..xo + block_size {
            for y in yo..yo + block_size {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    texture_cache: &TextureManager,
) {
    // minimapa del laberinto
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell, texture_cache);
        }
    }

    // jugador en minimapa
    framebuffer.set_current_color(Color::RED);
    framebuffer.set_pixel((player.pos.x / 5.0) as u32, (player.pos.y / 5.0) as u32);
}

fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    texture_cache: &TextureManager,
) -> Vec<f32> {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;
    let mut z_buffer = vec![f32::INFINITY; num_rays as usize];

    // Sky & floor
    for i in 0..framebuffer.width {
        framebuffer.set_current_color(Color::SKYBLUE);
        for j in 0..(framebuffer.height / 2) {
            framebuffer.set_pixel(i, j);
        }
        framebuffer.set_current_color(Color::LIGHTGREEN);
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.set_pixel(i, j);
        }
    }

    framebuffer.set_current_color(Color::WHITESMOKE);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        z_buffer[i as usize] = distance_to_wall;

        let distance_to_projection_plane = 70.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        for y in stake_top..stake_bottom {
            let ty = (y as f32 - stake_top as f32)
                / (stake_bottom as f32 - stake_top as f32)
                * 128.0;

            let color = cell_to_texture_color(
                texture_cache,
                intersect.impact,
                intersect.tx as u32,
                ty as u32,
            );
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i, y as u32);
        }
    }

    z_buffer
}


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");

    let file = BufReader::new(File::open("sounds/Karma.mp3").unwrap());
    let _sink = rodio::play(&stream_handle.mixer(), file).unwrap();

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.hide_cursor();
    window.disable_cursor();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // --- Texturas ---
    let texture_cache = TextureManager::new(&mut window, &raylib_thread);

    // --- Cargar imágenes de pantallas ---
    let start_screen = window.load_texture(&raylib_thread, "assets/start.png").unwrap();
    let end_screen = window.load_texture(&raylib_thread, "assets/end.png").unwrap();

    // --- Pantalla de inicio ---
    while !window.window_should_close() {
        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::BLACK);

        d.draw_texture_pro(
            &start_screen,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: start_screen.width() as f32,
                height: start_screen.height() as f32,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: window_width as f32,
                height: window_height as f32,
            },
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );

        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            break;
        }
    }

    // --- NEW: función para ejecutar un nivel ---
    fn run_level(
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        framebuffer: &mut Framebuffer,
        texture_cache: &TextureManager,
        block_size: usize,
        maze_file: &str,
    ) -> bool {
        let maze = load_maze(maze_file);
        let mut player = Player::new(Vector2::new(150.0, 150.0), PI / 3.0, PI / 3.0);

        // --- Sprites ---
        let mut sprites = Vec::new();
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                match cell {
                    'A' | 'B' | 'G' => {
                        let x = (col_index * block_size + block_size / 2) as f32;
                        let y = (row_index * block_size + block_size / 2) as f32;
                        if let Some(mut sprite) = Sprite::from_char(cell, x, y, &texture_cache) {
                            sprite.collected = false;
                            sprites.push(sprite);
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut game_won = false;
        while !window.window_should_close() && !game_won {
            framebuffer.clear();

            let old_x = player.pos.x;
            let old_y = player.pos.y;

            // Input
            process_events(&mut player, &window);

            // Colisiones con muros
            if is_wall(player.pos.x, old_y, &maze, block_size) {
                player.pos.x = old_x;
            }
            if is_wall(old_x, player.pos.y, &maze, block_size) {
                player.pos.y = old_y;
            }

            // Revisar si el jugador toca un sprite
            for sprite in &mut sprites {
                if !sprite.collected {
                    let dx = player.pos.x - sprite.x;
                    let dy = player.pos.y - sprite.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 80.0 {
                        sprite.collected = true;
                    }
                }
            }

            // ¿Ganó el nivel?
            game_won = sprites.iter().all(|s| s.collected);
            if game_won {
                break;
            }

            // Render paredes y obtener z-buffer
            let z_buffer = render_world(
                framebuffer,
                &maze,
                block_size,
                &player,
                texture_cache,
            );

            // Ordenar y dibujar sprites
            let mut sprites_sorted: Vec<_> = sprites
                .iter()
                .filter(|s| !s.collected)
                .cloned()
                .collect();
            sprites_sorted.sort_by(|a, b| {
                let da = (a.x - player.pos.x).powi(2) + (a.y - player.pos.y).powi(2);
                let db = (b.x - player.pos.x).powi(2) + (b.y - player.pos.y).powi(2);
                db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
            });
            render_sprites(framebuffer, &player, &sprites_sorted, &z_buffer);

            // Minimap
            render_maze(framebuffer, &maze, 20, &player, texture_cache);

            // Debug: sprites en minimapa
            for sprite in &sprites {
                if !sprite.collected {
                    let x = (sprite.x / 5.0) as u32;
                    let y = (sprite.y / 5.0) as u32;
                    framebuffer.set_current_color(Color::YELLOW);
                    framebuffer.set_pixel(x, y);
                }
            }

            framebuffer.swap_buffers(window, raylib_thread);
            thread::sleep(Duration::from_millis(16));
        }

        game_won
    }

    // --- Jugar nivel 1 ---
    let level1_won = run_level(&mut window, &raylib_thread, &mut framebuffer, &texture_cache, block_size, "maze.txt");

    // --- Jugar nivel 2 SOLO si completó nivel 1 ---
    let mut all_game_won = false;
    if level1_won {
        all_game_won = run_level(&mut window, &raylib_thread, &mut framebuffer, &texture_cache, block_size, "maze2.txt");
    }

    // --- Pantalla final ---
    if all_game_won {
        while !window.window_should_close() {
            let mut d = window.begin_drawing(&raylib_thread);
            d.clear_background(Color::BLACK);

            d.draw_texture_pro(
                &end_screen,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: end_screen.width() as f32,
                    height: end_screen.height() as f32,
                },
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: window_width as f32,
                    height: window_height as f32,
                },
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            d.draw_text(
                "¡Felicidades! Has completado los dos niveles.",
                350,
                800,
                28,
                Color::WHITE,
            );

            if d.is_key_pressed(KeyboardKey::KEY_ENTER) || d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                break;
            }
        }
    }
}
