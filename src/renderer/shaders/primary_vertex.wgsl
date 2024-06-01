struct CameraUniforms {
    view_projection: mat4x4<f32>
};

struct ModelUniforms {
    model: mat4x4<f32>
};

@group(0) @binding(0) var<uniform> camera_uniforms: CameraUniforms;
@group(1) @binding(0) var<uniform> model_uniforms: ModelUniforms; // Model (shape like Pyramid or import)

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec3<f32>
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec3<f32>
};

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let model_position = model_uniforms.model * vec4<f32>(input.position, 1.0);
    output.position = camera_uniforms.view_projection * model_position;
    output.color = input.color;
    output.normal = input.normal;
    output.tex_coords = input.tex_coords;
    return output;
}
