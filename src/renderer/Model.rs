use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::util::DeviceExt;

use gltf::buffer::{Source, View};
use gltf::Glb;
use gltf::Gltf;
use std::sync::Arc;

use crate::renderer::core::Vertex;
use crate::renderer::shapes::Pyramid::matrix4_to_raw_array;

pub struct Mesh {
    pub transform: Matrix4<f32>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub bind_group: wgpu::BindGroup,
    pub texture_bind_group: wgpu::BindGroup,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub async fn from_glb(
        bytes: &Vec<u8>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        texture_render_mode_buffer: &wgpu::Buffer,
        color_render_mode_buffer: &wgpu::Buffer,
    ) -> Self {
        // let response = reqwest::get(uri).await;
        // let bytes = response
        //     .expect("Response failed")
        //     .bytes()
        //     .await
        //     .expect("Couldnt fetch bytes")
        //     .to_vec();

        web_sys::console::log_1(&format!("Bytes len: {:?}", bytes.len()).into());

        let glb = Glb::from_slice(&bytes).expect("Couldn't create glb from slice");

        let mut meshes = Vec::new();

        let gltf = Gltf::from_slice(&glb.json).expect("Failed to parse GLTF JSON");

        let buffer_data = match glb.bin {
            Some(bin) => bin,
            None => panic!("No binary data found in GLB file"),
        };

        let uses_textures = gltf.textures().len().gt(&0);

        web_sys::console::log_1(&format!("Textures count: {:?}", gltf.textures().len()).into());

        let mut textures = Vec::new();
        for texture in gltf.textures() {
            match texture.source().source() {
                gltf::image::Source::View { view, mime_type: _ } => {
                    let img_data = &buffer_data[view.offset()..view.offset() + view.length()];
                    let img = image::load_from_memory(img_data).unwrap().to_rgba8();
                    let (width, height) = img.dimensions();

                    let size = wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    };

                    let texture = device.create_texture(&wgpu::TextureDescriptor {
                        label: Some("GLB Texture"),
                        size,
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[],
                    });

                    queue.write_texture(
                        wgpu::ImageCopyTexture {
                            texture: &texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        &img,
                        wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(4 * width), // TODO: is this correct?
                            rows_per_image: Some(height),
                        },
                        size,
                    );

                    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                        address_mode_u: wgpu::AddressMode::ClampToEdge,
                        address_mode_v: wgpu::AddressMode::ClampToEdge,
                        address_mode_w: wgpu::AddressMode::ClampToEdge,
                        mag_filter: wgpu::FilterMode::Linear,
                        min_filter: wgpu::FilterMode::Linear,
                        mipmap_filter: wgpu::FilterMode::Nearest,
                        ..Default::default()
                    });

                    textures.push((texture_view, sampler));
                }
                gltf::image::Source::Uri { uri, mime_type: _ } => {
                    panic!(
                        "External URI image sources are not yet supported in glb files: {}",
                        uri
                    );
                }
            }
        }

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

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffer_data));

                let positions = reader
                    .read_positions()
                    .expect("Positions not existing in glb");
                let colors = reader
                    .read_colors(0)
                    .map(|v| v.into_rgb_f32().collect())
                    .unwrap_or_else(|| vec![[1.0, 1.0, 1.0]; positions.len()]);
                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .map(|iter| iter.collect())
                    .unwrap_or_else(|| vec![[0.0, 0.0, 1.0]; positions.len()]);
                let tex_coords: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .map(|v| v.into_f32().collect())
                    .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);

                let vertices: Vec<Vertex> = positions
                    .zip(normals.iter())
                    .zip(tex_coords.iter())
                    .zip(colors.iter())
                    .map(|(((p, n), t), c)| Vertex {
                        position: p,
                        normal: *n,
                        tex_coords: *t,
                        color: *c,
                    })
                    .collect();

                let indices_u32: Vec<u32> = reader
                    .read_indices()
                    .map(|iter| iter.into_u32().collect())
                    .unwrap_or_default();

                let indices: Vec<u16> = indices_u32.iter().map(|&i| i as u16).collect();

                web_sys::console::log_1(&format!("Model vertices: {:?}", vertices.len()).into());
                web_sys::console::log_1(&format!("Model indices: {:?}", indices.len()).into());

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Model GLB Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                let index_buffer: wgpu::Buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Model GLB Index Buffer"),
                        contents: bytemuck::cast_slice(&indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

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

                let render_mode_buffer = if uses_textures {
                    texture_render_mode_buffer
                } else {
                    color_render_mode_buffer
                };

                // Handle the texture bind group conditionally
                let texture_bind_group = if uses_textures && !textures.is_empty() {
                    let material = primitive.material();
                    let texture_index = material
                        .pbr_metallic_roughness()
                        .base_color_texture()
                        .map_or(0, |info| info.texture().index());
                    let (texture_view, sampler) = &textures[texture_index];

                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &texture_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(texture_view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(sampler),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                    buffer: render_mode_buffer,
                                    offset: 0,
                                    size: None,
                                }),
                            },
                        ],
                        label: None,
                    })
                } else {
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                                    buffer: render_mode_buffer,
                                    offset: 0,
                                    size: None,
                                }),
                            },
                        ],
                        label: None,
                    })
                };

                meshes.push(Mesh {
                    transform: Matrix4::identity(),
                    vertex_buffer,
                    index_buffer,
                    index_count: indices.len() as u32,
                    bind_group,
                    texture_bind_group,
                });
            }
        }

        Model { meshes }
    }
}
