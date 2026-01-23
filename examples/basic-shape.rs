use porcelain::{Pivot, color::Color, conf::WindowConfig, render::EventListener, start};

struct AppState {}

impl EventListener for AppState {
    fn update(&mut self, _texture_context: &porcelain::texture::TextureContext, _dt: f64) {
        // no-op
    }

    fn draw(&self, draw_context: &mut porcelain::render::DrawContext) {
        draw_context.draw_rect_ext(
            Pivot { x: 200., y: 125. },
            150.,
            100.,
            25.,
            Color::from_rgba8(255, 0, 0, 255),
        );
        draw_context.draw_rect(
            1200.,
            300.,
            400.,
            200.,
            Color::from_rgba8(128, 128, 65, 255),
        );
        draw_context.draw_poly(
            Pivot { x: 300., y: 400. },
            200.,
            3,
            Color::from_rgba8(56, 122, 243, 255),
        );
        draw_context.draw_poly(
            Pivot { x: 600., y: 600. },
            200.,
            6,
            Color::from_rgba8(182, 35, 133, 255),
        );
        draw_context.draw_circle(
            Pivot { x: 600., y: 200. },
            75.,
            Color::from_rgba8(128, 128, 192, 255),
        );
        draw_context.draw_circle_arc(
            Pivot { x: 900., y: 300. },
            100.,
            90.,
            180.,
            Color::from_rgba8(92, 16, 73, 255),
        );
    }
}

fn main() {
    let window_config = WindowConfig {
        window_title: "BasicShape".to_owned(),
        fullscreen: true,
        ..Default::default()
    };

    let app_state = AppState {};

    start(window_config, app_state);
}
