// struct FragmentInput {
//     @location(0) color: vec3<f32>
// };

// @fragment
// fn main(input: FragmentInput) -> @location(0) vec4<f32> {
//     return vec4<f32>(input.color, 1.0);
// }

struct FragmentInput {
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec3<f32>
};

@group(2) @binding(0) var myTexture: texture_2d<f32>;
@group(2) @binding(1) var mySampler: sampler;
@group(2) @binding(2) var<uniform> renderMode: i32;

@fragment
fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    let texColor = textureSample(myTexture, mySampler, in.tex_coords);
    if (renderMode == 1) { // Assume 1 means rendering texture
        return texColor; // Texture rendering
    } else {
        return vec4(in.color, 1.0); // Color mode
    }
}