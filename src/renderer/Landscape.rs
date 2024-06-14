use image::io::Reader as ImageReader;
use nalgebra::{Matrix4, Vector3};
use rapier3d::math::Point;
use wgpu::util::{DeviceExt, TextureDataOrder};

use crate::renderer::core::Vertex;

use crate::renderer::Transform::{matrix4_to_raw_array, Transform};

use crate::renderer::core::LandscapeData;

pub struct Landscape {
    pub transform: Transform,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub bind_group: wgpu::BindGroup,
    pub texture_bind_group: wgpu::BindGroup,
}

impl Landscape {
    pub fn new(
        data: &LandscapeData,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        color_render_mode_buffer: &wgpu::Buffer,
    ) -> Self {
        // .tif heightmap load

        // Load the heightmap image
        // TODO: load image bytes from background

        // potential texture from the heightmap data
        // seems unnecessary
        // let heightmap_texture = device.create_texture_with_data(
        //     &queue,
        //     &wgpu::TextureDescriptor {
        //         label: Some("Heightmap Texture"),
        //         size: wgpu::Extent3d {
        //             width: heightmap.width(),
        //             height: heightmap.height(),
        //             depth_or_array_layers: 1,
        //         },
        //         mip_level_count: 1,
        //         sample_count: 1,
        //         dimension: wgpu::TextureDimension::D2,
        //         format: wgpu::TextureFormat::R16Uint,
        //         usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        //         view_formats: &[],
        //     },
        //     TextureDataOrder::MipMajor,
        //     &heightmap,
        // );

        // TODO: .png mask (soil, rocks) load

        // load actual vertices and indices (most important for now)
        let scale = 1.0;
        let (vertices, indices, rapier_vertices) = Self::generate_terrain(data, scale);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Landscape Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Landscape Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        // set uniform buffer for transforms
        let empty_buffer = Matrix4::<f32>::identity();
        let raw_matrix = matrix4_to_raw_array(&empty_buffer);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model GLB Uniform Buffer"),
            contents: bytemuck::cast_slice(&raw_matrix),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: None,
        });

        // creating default texture view and sampler just like in Model, for use when model has no textures, but uses same shader
        // Create a default empty texture and sampler
        let default_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Default Empty Texture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let default_texture_view =
            default_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // potential texture bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&default_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: color_render_mode_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
            label: None,
        });

        Self {
            index_count: indices.len() as u32,
            vertex_buffer,
            index_buffer,
            bind_group,
            texture_bind_group,
            transform: Transform::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 1.0, 1.0),
                uniform_buffer,
            ),
        }
    }

    // Generate vertex buffer from heightmap data
    pub fn generate_terrain(
        data: &LandscapeData,
        scale: f32,
    ) -> (Vec<Vertex>, Vec<u32>, Vec<Point<f32>>) {
        // let width = heightmap.width() as usize;
        // let height = heightmap.height() as usize;
        let mut vertices = Vec::with_capacity(data.width * data.height);
        let mut rapier_vertices = Vec::with_capacity(data.width * data.height);
        let mut indices = Vec::new();

        for y in 0..data.height {
            for x in 0..data.width {
                // let pixel = heightmap.get_pixel(x as u32, y as u32);
                // let height_value = pixel[0] as f32 / 255.0 * scale;
                // let position = [
                //     x as f32 / width as f32 * 2.0 - 1.0,
                //     height_value,
                //     y as f32 / height as f32 * 2.0 - 1.0,
                // ];
                // let tex_coords = [x as f32 / width as f32, y as f32 / height as f32];
                vertices.push(Vertex {
                    position: data.pixel_data[y][x].position,
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: data.pixel_data[y][x].tex_coords,
                    color: [1.0, 1.0, 1.0],
                });
                rapier_vertices.push(Point::new(
                    data.pixel_data[y][x].position[0],
                    data.pixel_data[y][x].position[1],
                    data.pixel_data[y][x].position[2],
                ));
            }
        }

        for y in 0..(data.height - 1) {
            for x in 0..(data.width - 1) {
                let top_left = (y * data.width + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = top_left + data.width as u32;
                let bottom_right = bottom_left + 1;

                // // Triangle 1
                // indices.push(top_left);
                // indices.push(bottom_left);
                // indices.push(top_right);

                // // Triangle 2
                // indices.push(top_right);
                // indices.push(bottom_left);
                // indices.push(bottom_right);

                // like everdaygui
                indices.push(top_left);
                indices.push(top_right);
                indices.push(bottom_right);

                indices.push(bottom_right);
                indices.push(bottom_left);
                indices.push(top_left);
            }
        }

        (vertices, indices, rapier_vertices)
    }
}
