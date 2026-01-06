use std::vec;

use crate::color::Color;
use crate::shader;
use crate::shader::Uniforms;
use miniquad::{
    Bindings, BufferLayout, EventHandler, Pipeline, PipelineParams, RenderingBackend,
    UniformsSource, VertexAttribute, conf, window,
};

pub trait EventListener {
    fn update(&mut self, dt: f64);
    fn draw(&self, draw_context: &mut DrawContext);
}

#[repr(C)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    tex_coord: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32, color: Color) -> Vertex {
        Vertex {
            pos: [x, y],
            color: color.into(),
            tex_coord: [u, v],
        }
    }
}

pub struct DrawContext {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u16>,
}

struct RendererState<T> {
    draw_context: DrawContext,

    last_update_time: f64,

    app_listener: T,

    pipeline: Pipeline,
    bindings: Bindings,
    context: Box<dyn RenderingBackend>,
}

impl DrawContext {
    pub fn new() -> DrawContext {
        DrawContext {
            vertex_buffer: Vec::with_capacity(10000),
            index_buffer: Vec::with_capacity(10000),
        }
    }

    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        #[rustfmt::skip]
        let vertices = [
            Vertex::new(    x,     y, 0., 0., color),
            Vertex::new(    x, y + h, 0., 0., color),
            Vertex::new(x + w,     y, 0., 0., color),
            Vertex::new(x + w, y + h, 0., 0., color)
        ];
        let indices = [0, 1, 3, 0, 3, 2];

        self.vertex_buffer.extend(vertices);
        self.index_buffer.extend(indices);
    }
}

impl<T: EventListener> RendererState<T> {
    fn new(app_listener: T) -> RendererState<T> {
        let mut context: Box<dyn RenderingBackend> = window::new_rendering_backend();
        let white_texture = context.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);

        let vertex_buffer = context.new_buffer(
            miniquad::BufferType::VertexBuffer,
            miniquad::BufferUsage::Stream,
            miniquad::BufferSource::empty::<Vertex>(4),
        );

        let index_buffer = context.new_buffer(
            miniquad::BufferType::IndexBuffer,
            miniquad::BufferUsage::Stream,
            miniquad::BufferSource::empty::<u16>(6),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![white_texture],
        };

        let shader = context
            .new_shader(
                match context.info().backend {
                    miniquad::Backend::OpenGl => miniquad::ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    miniquad::Backend::Metal => miniquad::ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = context.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", miniquad::VertexFormat::Float2),
                VertexAttribute::new("in_color", miniquad::VertexFormat::Float4),
                VertexAttribute::new("in_texcoord", miniquad::VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        RendererState {
            app_listener,
            pipeline,
            bindings,
            context,
            draw_context: DrawContext::new(),
            last_update_time: miniquad::date::now(),
        }
    }
}

impl<T: EventListener> EventHandler for RendererState<T> {
    fn update(&mut self) {
        let current_time = miniquad::date::now();
        self.app_listener
            .update(current_time - self.last_update_time);
        self.last_update_time = current_time;
    }

    fn draw(&mut self) {
        let (width, height) = miniquad::window::screen_size();
        let dpi = miniquad::window::dpi_scale();
        let uniforms = Uniforms {
            model: glam::Mat4::IDENTITY,
            projection: glam::Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.),
        };

        self.context.begin_default_pass(Default::default());

        self.context.apply_pipeline(&self.pipeline);
        self.context.apply_bindings(&self.bindings);

        self.app_listener.draw(&mut self.draw_context);

        self.context.buffer_update(
            self.bindings.vertex_buffers[0],
            miniquad::BufferSource::slice(&self.draw_context.vertex_buffer),
        );
        self.context.buffer_update(
            self.bindings.index_buffer,
            miniquad::BufferSource::slice(&self.draw_context.index_buffer),
        );

        self.context
            .apply_uniforms(UniformsSource::table(&uniforms));
        self.context.draw(0, 6, 1);

        self.context.end_render_pass();
        self.context.commit_frame();

        self.draw_context.clear();
    }
}

pub fn start<T: EventListener + 'static>(user_state: T) {
    let mut conf = conf::Conf::default();

    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(RendererState::new(user_state)));
}
