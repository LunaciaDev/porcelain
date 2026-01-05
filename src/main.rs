use miniquad::{
    Bindings, BufferLayout, EventHandler, Pipeline, PipelineParams, RenderingBackend,
    UniformsSource, VertexAttribute, conf, window,
};

use crate::shader::Uniforms;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    tex_coord: [f32; 2],
}

struct RendererState {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,

    pipeline: Pipeline,
    bindings: Bindings,
    context: Box<dyn RenderingBackend>,
}

impl RendererState {
    pub fn new() -> RendererState {
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

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: [0., 0.],
                color: [0., 0., 0., 255.],
                tex_coord: [0., 0.],
            },
            Vertex {
                pos: [600., 0.],
                color: [255., 0., 0., 255.],
                tex_coord: [0., 0.],
            },
            Vertex {
                pos: [0., 600.],
                color: [0., 255., 0., 255.],
                tex_coord: [0., 0.],
            },
            Vertex {
                pos: [600., 600.],
                color: [0., 0., 255., 255.],
                tex_coord: [0., 0.],
            },
        ];
        let indices = vec![0, 1, 2, 1, 2, 3];

        RendererState {
            #[rustfmt::skip]
            vertices,
            indices,
            pipeline,
            bindings,
            context,
        }
    }
}

impl EventHandler for RendererState {
    fn update(&mut self) {
        self.vertices.iter_mut().for_each(|vertex| {
            vertex.pos[0] += 1.;
        });
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
        self.context.buffer_update(
            self.bindings.vertex_buffers[0],
            miniquad::BufferSource::slice(&self.vertices),
        );
        self.context.buffer_update(
            self.bindings.index_buffer,
            miniquad::BufferSource::slice(&self.indices),
        );
        self.context
            .apply_uniforms(UniformsSource::table(&uniforms));
        self.context.draw(0, 6, 1);

        self.context.end_render_pass();
        self.context.commit_frame();
    }
}

mod shader;

fn main() {
    let mut conf = conf::Conf::default();

    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(RendererState::new()));
}
