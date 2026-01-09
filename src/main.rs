use miniquad::TextureId;

use porcelain::{
    color::Color,
    conf::WindowConfig,
    render::{DrawContext, EventListener},
    texture::TextureContext,
};

struct AppState {
    rect_pos: [f32; 2],
    rect_size: [f32; 2],
    rect_texture: Option<TextureId>,
}

impl EventListener for AppState {
    fn update(&mut self, texture_context: &TextureContext, dt: f64) {
        self.rect_pos[0] += (10. * dt) as f32;

        if self.rect_texture.is_none() {
            self.rect_texture = Some(texture_context.register_texture_rgb8(1, 1, &[255, 255, 255]));
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.draw_rect_textured(
            self.rect_pos[0],
            self.rect_pos[1],
            self.rect_size[0],
            self.rect_size[1],
            self.rect_texture.unwrap(),
            Color::from_rgba8(255, 255, 255, 255),
        );
    }
}

fn main() {
    let window_config = WindowConfig {
        window_title: "App".to_string(),
        ..Default::default()
    };

    let app_state = AppState {
        rect_pos: [0., 0.],
        rect_size: [600., 600.],
        rect_texture: None,
    };

    porcelain::start(window_config, app_state);
}
