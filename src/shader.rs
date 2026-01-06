use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};

pub const VERTEX: &str = r"
#version 100
attribute vec2 in_pos;
attribute vec4 in_color;
attribute vec2 in_texcoord;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 model;
uniform mat4 projection;

void main() {
    gl_Position = projection * model * vec4(in_pos, 0, 1);
    uv = in_texcoord;
    color = in_color;
}";

pub const FRAGMENT: &str = r"
#version 100
varying lowp vec2 uv;
varying lowp vec4 color;

uniform sampler2D texture;

void main() {
    gl_FragColor = color * texture2D(texture, uv);
}";

pub const METAL: &str = r"
#include <metal_stdlib>

using namespace metal;

struct Uniforms
{
    float4x4 model;
    float4x4 projection;
};

struct Vertex
{
    float2 in_pos   [[attribute(0)]];
    float4 in_color [[attribute(1)]];
    float2 in_texcoord [[attribute(2)]];
};

struct RasterizerData
{
    float4 position [[position]];
    float4 color [[user(locn0)]];
    float2 uv [[user(locn1)]];
};

vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& uniforms [[buffer(0)]])
{
    RasterizerData out;

    out.position = uniforms.projection * uniforms.model * float4(v.in_pos, 0, 1);
    out.color = v.in_color;
    out.uv = v.in_texcoord

    return out;
}

fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
{
    return in.color * tex.sample(texSmplr, in.uv);
}";

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["texture".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("model", UniformType::Mat4),
                UniformDesc::new("projection", UniformType::Mat4),
            ],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub model: glam::Mat4,
    pub projection: glam::Mat4
}