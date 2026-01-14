use crate::{
    conf::WindowConfig,
    render::{EventListener, RendererContext},
};

pub mod color;
pub mod conf;
pub mod render;
mod shader;
pub mod texture;

pub fn start<T: EventListener + 'static>(config: WindowConfig, user_state: T) {
    let draw_call_limit = config.draw_call_size_limit;
    let mut conf: miniquad::conf::Conf = config.into();

    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        miniquad::conf::AppleGfxApi::Metal
    } else {
        miniquad::conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || {
        Box::new(RendererContext::new(draw_call_limit, user_state))
    });
}
