use miniquad::conf::{Conf, Platform};

pub struct WindowConfig {
    pub window_title: String,
    pub window_width: i32,
    pub window_height: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub resizable: bool,

    // [TODO] Separate this into a struct if more field is needed
    pub draw_call_size_limit: usize,
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

            draw_call_size_limit: 10000,
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
