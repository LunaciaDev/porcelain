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
    rect_rot: f32,
    rect_texture: Option<TextureId>,
}

impl EventListener for AppState {
    fn update(&mut self, texture_context: &TextureContext, dt: f64) {
        self.rect_rot += 10. * (dt as f32);

        if self.rect_texture.is_none() {
            self.rect_texture = Some(texture_context.register_texture_rgb8(1, 1, &[255, 255, 255]));
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.draw_rect_ext(
            self.rect_pos[0],
            self.rect_pos[1],
            self.rect_size[0],
            self.rect_size[1],
            self.rect_rot,
            Color::from_rgba8(255, 255, 255, 255),
        );

        draw_context.draw_rect_ext(
            self.rect_pos[0] + 200.,
            self.rect_pos[1] + 200.,
            self.rect_size[0],
            self.rect_size[1],
            self.rect_rot,
            Color::from_rgba8(255, 0, 0, 255),
        );
    }
}

fn main() {
    let window_config = WindowConfig {
        window_title: "App".to_string(),
        draw_call_size_limit: 5,
        ..Default::default()
    };

    let app_state = AppState {
        rect_pos: [200., 200.],
        rect_size: [200., 200.],
        rect_rot: 0.,
        rect_texture: None,
    };

    porcelain::start(window_config, app_state);
}
