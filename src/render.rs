use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    color::Color,
    conf::WindowConfig,
    shader::{self, Uniforms},
    texture::TextureContext,
};
use glam::{Affine2, Mat4, Vec2};
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

pub struct TextureArea {
    pub texture: TextureId,
    pub location: Vec2,
    pub size: Vec2,
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

    max_vertex_per_call: usize,
    max_index_per_call: usize,
}

pub struct RendererContext<T> {
    draw_context: DrawContext,
    texture_context: TextureContext,

    last_update_time: f64,

    app_listener: T,

    pipeline: Pipeline,
    bindings: Bindings,
    uniform: Uniforms,
    backend: Rc<RefCell<Box<dyn RenderingBackend>>>,
}

impl DrawCall {
    fn new(texture: TextureId, vertex_offset: usize, index_offset: usize) -> Self {
        Self {
            texture,
            vertex_indices_slice: VecSlice {
                offset: vertex_offset,
                length: 0,
            },
            index_indices_slice: VecSlice {
                offset: index_offset,
                length: 0,
            },
        }
    }
}

impl DrawContext {
    fn new(
        default_texture: TextureId,
        max_vertex_per_call: usize,
        max_index_per_call: usize,
    ) -> Self {
        Self {
            // Pre-allocate for 5k vertices; This can be extended, the limit is per draw-call.
            vertex_buffer: Vec::with_capacity(5000),
            index_buffer: Vec::with_capacity(5000),
            draw_call_vec: Vec::new(),
            max_vertex_per_call,
            max_index_per_call,
            default_texture,
        }
    }

    fn create_draw_call(&mut self, vertices: Box<[Vertex]>, indices: &[u16], texture: TextureId) {
        // throw if a single call is too large
        // [TODO] Since the lib's draw call will never exceed this size, move the check to user-supplied vertices
        assert!(vertices.len() < self.max_vertex_per_call);
        assert!(indices.len() < self.max_index_per_call);

        match self.draw_call_vec.last() {
            Some(draw_call) => {
                if draw_call.texture != texture
                    || draw_call.vertex_indices_slice.length + vertices.len()
                        > self.max_vertex_per_call
                    || draw_call.index_indices_slice.length + indices.len()
                        > self.max_index_per_call
                {
                    self.draw_call_vec.push(DrawCall::new(
                        texture,
                        self.vertex_buffer.len(),
                        self.index_buffer.len(),
                    ));
                }
            }
            None => self.draw_call_vec.push(DrawCall::new(
                texture,
                self.vertex_buffer.len(),
                self.index_buffer.len(),
            )),
        }

        let current_draw_call = self
            .draw_call_vec
            .last_mut()
            .expect("A draw call must have been created before");
        current_draw_call.index_indices_slice.length += indices.len();
        self.index_buffer.extend(
            indices
                .iter()
                .map(|index| index + current_draw_call.vertex_indices_slice.length as u16),
        );
        current_draw_call.vertex_indices_slice.length += vertices.len();
        self.vertex_buffer.extend(vertices);
    }

    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
        self.draw_call_vec.clear();
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        #[rustfmt::skip]
        let vertices = Box::new([
            Vertex::new(    x,     y, 0., 0., color),
            Vertex::new(    x, y + h, 0., 0., color),
            Vertex::new(x + w,     y, 0., 0., color),
            Vertex::new(x + w, y + h, 0., 0., color)
        ]);
        let indices: [u16; 6] = [0, 1, 3, 0, 3, 2];

        self.create_draw_call(vertices, &indices, self.default_texture);
    }

    pub fn draw_rect_ext(&mut self, x: f32, y: f32, w: f32, h: f32, rotation: f32, color: Color) {
        let transform_matrix =
            Affine2::from_angle_translation(rotation.to_radians(), Vec2::new(x, y));
        #[rustfmt::skip]
        let vertices = [
            Vec2::new(-w/2., -h/2.),
            Vec2::new(-w/2.,  h/2.),
            Vec2::new( w/2., -h/2.),
            Vec2::new( w/2.,  h/2.),
        ]
        .iter()
        .map(|point| {
            let transformed_point = transform_matrix.transform_point2(*point);
            Vertex::new(transformed_point.x, transformed_point.y, 0., 0., color)
        })
        .collect::<Vec<Vertex>>()
        .into_boxed_slice();
        let indices: [u16; 6] = [0, 1, 3, 0, 3, 2];

        self.create_draw_call(vertices, &indices, self.default_texture);
    }

    pub fn draw_texture(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        texture: TextureArea,
        tint: Color,
    ) {
        #[rustfmt::skip]
        let vertices = Box::new([
            Vertex::new(    x,     y,                  texture.location.x,                  texture.location.y, tint),
            Vertex::new(    x, y + h,                  texture.location.x, texture.location.y + texture.size.y, tint),
            Vertex::new(x + w,     y, texture.location.x + texture.size.x,                  texture.location.y, tint),
            Vertex::new(x + w, y + h, texture.location.x + texture.size.x, texture.location.y + texture.size.y, tint)
        ]);
        let indices: [u16; 6] = [0, 1, 3, 0, 3, 2];

        self.create_draw_call(vertices, &indices, texture.texture);
    }
}

impl<T: EventListener> RendererContext<T> {
    pub fn new(config: WindowConfig, app_listener: T) -> RendererContext<T> {
        let backend = Rc::new(RefCell::new(window::new_rendering_backend()));

        let backend_info = backend.borrow().info().backend;
        let mut backend_mut = backend.borrow_mut();

        let white_texture = backend_mut.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);

        let vertex_buffer = backend_mut.new_buffer(
            miniquad::BufferType::VertexBuffer,
            miniquad::BufferUsage::Stream,
            miniquad::BufferSource::empty::<Vertex>(config.max_vertices_per_draw),
        );

        let index_buffer = backend_mut.new_buffer(
            miniquad::BufferType::IndexBuffer,
            miniquad::BufferUsage::Stream,
            miniquad::BufferSource::empty::<u16>(config.max_indices_per_draw),
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

        let (width, height) = miniquad::window::screen_size();
        let dpi = miniquad::window::dpi_scale();

        RendererContext {
            draw_context: DrawContext::new(
                white_texture,
                config.max_vertices_per_draw,
                config.max_indices_per_draw,
            ),
            texture_context: TextureContext::new(backend.clone()),
            app_listener,
            uniform: Uniforms {
                model: Mat4::IDENTITY,
                projection: Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.),
            },
            pipeline,
            bindings,
            backend,
            last_update_time: miniquad::date::now(),
        }
    }
}

impl<T: EventListener> EventHandler for RendererContext<T> {
    fn resize_event(&mut self, width: f32, height: f32) {
        let dpi = miniquad::window::dpi_scale();

        self.uniform.projection =
            Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.);
    }

    fn update(&mut self) {
        let current_time = miniquad::date::now();
        self.app_listener
            .update(&self.texture_context, current_time - self.last_update_time);
        self.last_update_time = current_time;
    }

    fn draw(&mut self) {
        self.app_listener.draw(&mut self.draw_context);

        let mut context = self.backend.borrow_mut();

        // [TODO] Expose the clear color to the user
        // Technically can be exposed via drawing a rect but
        context.clear(Some((0., 0., 0., 255.)), None, None);

        for draw_call in &self.draw_context.draw_call_vec {
            context.begin_default_pass(miniquad::PassAction::Nothing);

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
            context.apply_uniforms(UniformsSource::table(&self.uniform));

            context.draw(0, draw_call.index_indices_slice.length as i32, 1);

            context.end_render_pass();
        }

        context.commit_frame();
        self.draw_context.clear();
    }
}
