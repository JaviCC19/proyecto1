use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    pub(crate) images: HashMap<char, Image>, // Imágenes en CPU para lectura de píxeles
}

impl TextureManager {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();

        // Mapear caracteres a rutas de textura
        let texture_files = vec![
            // Walls
            ('+', "assets/wall.png"),
            ('-', "assets/wall.png"),
            ('|', "assets/wall.png"),
            ('p', "assets/pokeball.jpg"),

            // Sprites
            ('A', "assets/squirtle.jpg"),
            ('B', "assets/charmeleon.jpg"),
            ('G', "assets/bulbsaur.jpg"),
        ];

        for (ch, path) in texture_files {
            // Cargar imagen en CPU
            let mut image = Image::load_image(path)
                .unwrap_or_else(|_| panic!("Failed to load image {}", path));

            // Forzar formato a RGBA8 para acceso directo a píxeles
            image.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);

            images.insert(ch, image);
        }

        TextureManager { images }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            Color::WHITE
        }
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let x = x as usize;
    let y = y as usize;

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);
        let idx = (y * width + x) * 4;

        if idx + 3 >= data_len {
            return Color::WHITE;
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}
