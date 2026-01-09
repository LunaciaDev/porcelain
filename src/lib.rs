use crate::{
    conf::WindowConfig,
    render::{EventListener, RendererContext},
};

pub mod color;
pub mod conf;
pub mod render;
pub mod texture;
mod shader;

pub fn start<T: EventListener + 'static>(config: WindowConfig, user_state: T) {
    let mut conf: miniquad::conf::Conf = config.into();

    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        miniquad::conf::AppleGfxApi::Metal
    } else {
        miniquad::conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(RendererContext::new(user_state)));
}
