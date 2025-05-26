use macroquad::prelude::*;

use crate::data::PALETTE;

pub const WIDTH: u8 = 240;
pub const HEIGHT: u8 = 135;

pub struct Scene {
    image: Image,
    texture: Texture2D,
}

impl Scene {
    pub fn new() -> Self {
        let image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest); // Para pixel art

        Scene { image, texture }
    }

    pub fn update(&mut self) {
        self.texture.update(&self.image);

        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, color_id: usize) {
        let i = ((y as usize * WIDTH as usize + x as usize) * 4) as usize;

        self.image.bytes[i] = PALETTE[color_id][0];
        self.image.bytes[i + 1] = PALETTE[color_id][1];
        self.image.bytes[i + 2] = PALETTE[color_id][2];
        self.image.bytes[i + 3] = PALETTE[color_id][3];
    }
}
