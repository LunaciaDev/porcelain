use crate::{
    conf::WindowConfig,
    render::{EventListener, RendererContext},
};

pub mod color;
pub mod conf;
pub mod render;
mod shader;
pub mod texture;

pub struct Point {
    pub x: f32,
    pub y: f32
}

pub fn start<T: EventListener + 'static>(config: WindowConfig, user_state: T) {
    let config_cloned = config.clone();
    let mut conf: miniquad::conf::Conf = config.into();

    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        miniquad::conf::AppleGfxApi::Metal
    } else {
        miniquad::conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || {
        Box::new(RendererContext::new(config_cloned, user_state))
    });
}
