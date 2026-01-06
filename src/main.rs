
mod color;
mod shader;
mod render;

use crate::{color::Color, render::{DrawContext, EventListener, start}};

struct AppState {
    rect_pos: [f32; 2],
    rect_size: [f32; 2],
}

impl EventListener for AppState {
    fn update(&mut self, dt: f64) {
        self.rect_pos[0] += (10. * dt) as f32;
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.draw_rect(
            self.rect_pos[0],
            self.rect_pos[1],
            self.rect_size[0],
            self.rect_size[1],
            Color::from_rgba8(128, 0, 0, 255),
        );
    }
}

fn main() {
    let app_state: AppState = AppState {
        rect_pos: [0., 0.],
        rect_size: [600., 600.],
    };

    start(app_state);
}
