// struct FragmentInput {
//     @location(0) normal: vec3<f32>,
//     @location(1) tex_coords: vec2<f32>,
//     @location(2) color: vec3<f32>
// };

// @group(2) @binding(0) var t_diffuse: texture_2d_array<f32>;
// @group(2) @binding(1) var s_diffuse: sampler;
// @group(2) @binding(2) var<uniform> renderMode: i32;

// @fragment
// fn main(in: FragmentInput) -> @location(0) vec4<f32> {
//     let tiling_factor: f32 = 10.0; // Adjust this to control tiling density
//     let tiled_tex_coords = fract(in.tex_coords * tiling_factor);

//     let primary = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 0);
//     let primary_mask = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 1);
//     let rockmap = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 2);
//     let rockmap_mask = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 3);
//     let soil = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 4);
//     let soil_mask = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 5);
    
//     // Blend textures based on their respective masks
//     let primary_contribution = primary * primary_mask.r;
//     let rockmap_contribution = rockmap * rockmap_mask.r;
//     let soil_contribution = soil * soil_mask.r;
    
//     // Combine all contributions
//     let total_mask = primary_mask.r + rockmap_mask.r + soil_mask.r;
//     let final_color = (primary_contribution + rockmap_contribution + soil_contribution) / max(total_mask, 1.0);
    
//     // return vec4<f32>(final_color.rgb, 1.0);

//     // if (renderMode == 1) { // Assume 1 means rendering texture
//         return vec4<f32>(final_color.rgb, 1.0); // Texture rendering
//     // } else {
//     //     return vec4(in.color, 1.0); // Color mode
//     // }
// }

// fn get_mask_value(mask: vec4<f32>) -> f32 {
//     return max(mask.r, max(mask.g, max(mask.b, mask.a)));
// }

// @fragment
// fn main(in: FragmentInput) -> @location(0) vec4<f32> {
//     let tiling_factor: f32 = 10.0;
//     let tiled_tex_coords = fract(in.tex_coords * tiling_factor);

//     let primary = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 0);
//     let primary_mask = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 1);
//     let rockmap = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 2);
//     let rockmap_mask = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 3);
//     let soil = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 4);
//     let soil_mask = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 5);
    
//     let primary_mask_value = get_mask_value(primary_mask);
//     let rockmap_mask_value = get_mask_value(rockmap_mask);
//     let soil_mask_value = get_mask_value(soil_mask);

//     // Blend textures based on their respective masks
//     let primary_contribution = primary.rgb * primary_mask_value;
//     let rockmap_contribution = rockmap.rgb * rockmap_mask_value;
//     let soil_contribution = soil.rgb * soil_mask_value;
    
//     // Combine all contributions and normalize
//     let total_mask = primary_mask_value + rockmap_mask_value + soil_mask_value;
//     var final_color: vec3<f32>;
//     if total_mask > 0.0 {
//         final_color = (primary_contribution + rockmap_contribution + soil_contribution) / total_mask;
//     } else {
//         final_color = vec3<f32>(0.5, 0.5, 0.5); // Default color if no mask
//     }

//     return vec4<f32>(final_color, 1.0);
// }

struct FragmentInput {
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec3<f32>
};

@group(2) @binding(0) var t_diffuse: texture_2d_array<f32>;
@group(2) @binding(1) var s_diffuse: sampler;
@group(2) @binding(2) var<uniform> renderMode: i32;

@fragment
fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    let tiling_factor: f32 = 10.0;
    let tiled_tex_coords = fract(in.tex_coords * tiling_factor);

    let primary = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 0);
    let primary_mask = textureSample(t_diffuse, s_diffuse, in.tex_coords, 1).r;
    let rockmap = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 2);
    let rockmap_mask = textureSample(t_diffuse, s_diffuse, in.tex_coords, 3).r;
    let soil = textureSample(t_diffuse, s_diffuse, tiled_tex_coords, 4);
    let soil_mask = textureSample(t_diffuse, s_diffuse, in.tex_coords, 5).r;
    
    // Normalize masks
    let total_mask = primary_mask + rockmap_mask + soil_mask;
    let primary_weight = primary_mask / max(total_mask, 0.001);
    let rockmap_weight = rockmap_mask / max(total_mask, 0.001);
    let soil_weight = soil_mask / max(total_mask, 0.001);

    // Blend textures based on normalized weights
    let final_color = primary.rgb * primary_weight + 
                      rockmap.rgb * rockmap_weight + 
                      soil.rgb * soil_weight;

    return vec4<f32>(final_color, 1.0);
}