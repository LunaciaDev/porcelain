use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    color::Color,
    shader::{self, Uniforms},
    texture::TextureContext,
};
use miniquad::{
    Bindings, BufferLayout, EventHandler, Pipeline, PipelineParams, RenderingBackend, TextureId,
    UniformsSource, VertexAttribute, window,
};

pub trait EventListener {
    fn update(&mut self, texture_context: &TextureContext, dt: f64);
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

#[derive(Default)]
struct VecSlice {
    offset: usize,
    length: usize,
}

struct DrawCall {
    vertex_indices_slice: VecSlice,
    index_indices_slice: VecSlice,

    texture: TextureId,
}

pub struct DrawContext {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u16>,
    draw_call_vec: Vec<DrawCall>,
    default_texture: TextureId,
}

pub struct RendererContext<T> {
    draw_context: DrawContext,
    texture_context: TextureContext,

    last_update_time: f64,

    app_listener: T,

    pipeline: Pipeline,
    bindings: Bindings,
    backend: Rc<RefCell<Box<dyn RenderingBackend>>>,
}

impl DrawCall {
    fn new(texture: TextureId) -> Self {
        Self {
            texture,
            vertex_indices_slice: Default::default(),
            index_indices_slice: Default::default(),
        }
    }
}

impl DrawContext {
    fn new(default_texture: TextureId) -> Self {
        Self {
            vertex_buffer: Vec::with_capacity(10000),
            index_buffer: Vec::with_capacity(10000),
            draw_call_vec: Vec::new(),
            default_texture,
        }
    }

    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
        self.draw_call_vec.clear();
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.draw_rect_textured(x, y, w, h, self.default_texture, color);
    }

    pub fn draw_rect_textured(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        texture: TextureId,
        color: Color,
    ) {
        #[rustfmt::skip]
        let vertices = [
            Vertex::new(    x,     y, 0., 0., color),
            Vertex::new(    x, y + h, 0., 0., color),
            Vertex::new(x + w,     y, 0., 0., color),
            Vertex::new(x + w, y + h, 0., 0., color)
        ];
        let mut indices: [u16; 6] = [0, 1, 3, 0, 3, 2];

        match self.draw_call_vec.last() {
            Some(draw_call) => {
                if draw_call.texture != texture {
                    self.draw_call_vec.push(DrawCall::new(texture));
                }
            }
            None => self.draw_call_vec.push(DrawCall::new(texture)),
        }

        let current_draw_call = self
            .draw_call_vec
            .last_mut()
            .expect("A draw call is created before if empty");
        current_draw_call.index_indices_slice.length += indices.len();
        for index in indices.iter_mut() {
            // vertex_indices length should be clamped shorter than u16
            *index += current_draw_call.vertex_indices_slice.length as u16;
        }
        self.index_buffer.extend(indices);
        current_draw_call.vertex_indices_slice.length += vertices.len();
        self.vertex_buffer.extend(vertices);
    }
}

impl<T: EventListener> RendererContext<T> {
    pub fn new(app_listener: T) -> RendererContext<T> {
        let backend = Rc::new(RefCell::new(window::new_rendering_backend()));

        let backend_info = backend.borrow().info().backend;
        let mut backend_mut = backend.borrow_mut();

        let white_texture = backend_mut.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);

        let vertex_buffer = backend_mut.new_buffer(
            miniquad::BufferType::VertexBuffer,
            miniquad::BufferUsage::Stream,
            miniquad::BufferSource::empty::<Vertex>(4),
        );

        let index_buffer = backend_mut.new_buffer(
            miniquad::BufferType::IndexBuffer,
            miniquad::BufferUsage::Stream,
            miniquad::BufferSource::empty::<u16>(6),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![white_texture],
        };

        let shader = backend_mut
            .new_shader(
                match backend_info {
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

        let pipeline = backend_mut.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", miniquad::VertexFormat::Float2),
                VertexAttribute::new("in_color", miniquad::VertexFormat::Float4),
                VertexAttribute::new("in_texcoord", miniquad::VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        // Drop context so we can use the immutable ref
        drop(backend_mut);

        RendererContext {
            draw_context: DrawContext::new(white_texture),
            texture_context: TextureContext::new(backend.clone()),
            app_listener,
            pipeline,
            bindings,
            backend,
            last_update_time: miniquad::date::now(),
        }
    }
}

impl<T: EventListener> EventHandler for RendererContext<T> {
    fn update(&mut self) {
        let current_time = miniquad::date::now();
        self.app_listener
            .update(&self.texture_context, current_time - self.last_update_time);
        self.last_update_time = current_time;
    }

    fn draw(&mut self) {
        self.app_listener.draw(&mut self.draw_context);

        let mut context = self.backend.borrow_mut();
        let (width, height) = miniquad::window::screen_size();
        let dpi = miniquad::window::dpi_scale();
        let uniforms = Uniforms {
            model: glam::Mat4::IDENTITY,
            projection: glam::Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.),
        };

        for draw_call in &self.draw_context.draw_call_vec {
            context.begin_default_pass(Default::default());

            context.buffer_update(
                self.bindings.vertex_buffers[0],
                miniquad::BufferSource::slice(
                    &self.draw_context.vertex_buffer[draw_call.vertex_indices_slice.offset
                        ..(draw_call.vertex_indices_slice.offset
                            + draw_call.vertex_indices_slice.length)],
                ),
            );
            context.buffer_update(
                self.bindings.index_buffer,
                miniquad::BufferSource::slice(
                    &self.draw_context.index_buffer[draw_call.index_indices_slice.offset
                        ..(draw_call.index_indices_slice.offset
                            + draw_call.index_indices_slice.length)],
                ),
            );
            self.bindings.images[0] = draw_call.texture;

            context.apply_pipeline(&self.pipeline);
            context.apply_bindings(&self.bindings);
            context.apply_uniforms(UniformsSource::table(&uniforms));

            context.draw(0, draw_call.index_indices_slice.length as i32, 1);

            context.end_render_pass();
        }

        context.commit_frame();
        self.draw_context.clear();
    }
}
