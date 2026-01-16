use miniquad::conf::{Conf, Platform};

#[derive(Clone)]
pub struct WindowConfig {
    pub window_title: String,
    pub window_width: i32,
    pub window_height: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub resizable: bool,

    pub max_vertices_per_draw: usize,
    pub max_indices_per_draw: usize,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            window_title: "Porcelain App".to_string(),
            window_width: 800,
            window_height: 600,
            high_dpi: false,
            fullscreen: false,
            resizable: false,

            max_vertices_per_draw: 10000,
            max_indices_per_draw: 30000
        }
    }
}

impl From<WindowConfig> for Conf {
    fn from(value: WindowConfig) -> Self {
        Self {
            window_title: value.window_title,
            window_width: value.window_width,
            window_height: value.window_height,
            high_dpi: value.high_dpi,
            fullscreen: value.fullscreen,
            sample_count: 1,
            window_resizable: value.resizable,
            icon: None,
            platform: Platform::default(),
        }
    }
}
