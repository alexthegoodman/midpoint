use nalgebra::{Matrix4, Point3, Vector3};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{self, ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::renderer::SimpleCamera::SimpleCamera;

use bytemuck::{Pod, Zeroable};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
// use winit::event::{ElementState, WindowEvent};
// use winit::keyboard::{KeyCode, PhysicalKey};

use gltf::buffer::{Source, View};
use gltf::Glb;
use gltf::Gltf;
use std::sync::Arc;
// use std::error::Error;
// use std::fs::File;
// use std::io::BufReader;

pub struct Mesh {
    // vertices: Vec<Vertex>,
    // indices: Vec<u32>, // TODO: not used? no reason to store here
    transform: Matrix4<f32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup,
}

pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    pub async fn from_glb(
        uri: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        texture_render_mode_buffer: &wgpu::Buffer,
        color_render_mode_buffer: &wgpu::Buffer,
    ) -> Self {
        let response = reqwest::get(uri).await;
        let bytes = response
            .expect("Response failed")
            .bytes()
            .await
            .expect("Couldnt fetch bytes")
            .to_vec();

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

                // web_sys::console::log_1(&format!("Model positions: {:?}", positions.len()).into());
                // web_sys::console::log_1(&format!("Model colors: {:?}", colors.len()).into());
                // web_sys::console::log_1(&format!("Model normals: {:?}", normals.len()).into());
                // web_sys::console::log_1(
                //     &format!("Model tex_coords: {:?}", tex_coords.len()).into(),
                // );

                // if uses_textures {
                //     assert_eq!(
                //         positions.len(),
                //         tex_coords.len(),
                //         "Positions and tex_coords must have the same length"
                //     );
                // } else {
                //     assert_eq!(
                //         positions.len(),
                //         colors.len(),
                //         "Positions and colors must have the same length"
                //     );
                // }

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
                    // vertices: vertices.clone(),
                    // indices: indices.clone(), // TODO: expensive?
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
    color: [f32; 3],
}

// Ensure Vertex is Pod and Zeroable
unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32x3];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Grid {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

impl Grid {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        color_render_mode_buffer: &wgpu::Buffer,
    ) -> Self {
        // Generate grid vertices and indices
        let (vertices, indices) = Self::generate_grid(100.0, 100.0, 1.0); // example dimensions and spacing

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let empty_buffer = Matrix4::<f32>::identity();
        let raw_matrix = matrix4_to_raw_array(&empty_buffer);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Uniform Buffer"),
            contents: bytemuck::cast_slice(&raw_matrix),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // TODO: fragment bind group to provide render mode?

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                // wgpu::BindGroupEntry {
                //     binding: 1,
                //     resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                //         buffer: color_render_mode_buffer,
                //         offset: 0,
                //         size: None,
                //     }),
                // },
            ],
            label: None,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            bind_group,
        }
    }

    fn generate_grid(width: f32, depth: f32, spacing: f32) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let half_width = width / 2.0;
        let half_depth = depth / 2.0;

        for i in 0..=((width / spacing) as u16) {
            let x = -half_width + i as f32 * spacing;
            vertices.push(Vertex {
                position: [x, 0.0, -half_depth],
                normal: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                color: [1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x, 0.0, half_depth],
                normal: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                color: [1.0, 1.0, 1.0],
            });
            indices.push(i * 2);
            indices.push(i * 2 + 1);
        }

        let base = vertices.len() as u16;
        for i in 0..=((depth / spacing) as u16) {
            let z = -half_depth + i as f32 * spacing;
            vertices.push(Vertex {
                position: [-half_width, 0.0, z],
                normal: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                color: [1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: [half_width, 0.0, z],
                normal: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                color: [1.0, 1.0, 1.0],
            });
            indices.push(base + i * 2);
            indices.push(base + i * 2 + 1);
        }

        // web_sys::console::log_1(&format!("Grid vertices: {:?}", vertices).into());
        // web_sys::console::log_1(&format!("Grid indices: {:?}", indices).into());

        (vertices, indices)
    }

    // pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //     render_pass.set_bind_group(0, &self.bind_group, &[]);
    //     render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    // }
}

struct Pyramid {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Pyramid {
    fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pyramid Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pyramid Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let empty_buffer = Matrix4::<f32>::identity();
        let raw_matrix = matrix4_to_raw_array(&empty_buffer);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pyramid Uniform Buffer"),
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

        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
        }
    }

    fn update_transform(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation =
            Matrix4::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        // web_sys::console::log_1(&format!("Pyramid translation: {:?}", translation).into());
        translation * rotation * scale
    }

    fn update_uniform_buffer(&self, queue: &wgpu::Queue) {
        let transform_matrix = self.update_transform();
        let transform_matrix = transform_matrix.transpose(); // Transpose to match wgpu layout
        let raw_matrix = matrix4_to_raw_array(&transform_matrix);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&raw_matrix));
    }

    fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
        // web_sys::console::log_1(&format!("Pyramid position: {:?}", self.position).into());
        // alternative translation method
        // let translation_matrix = Matrix4::new_translation(&translation);
        // let translation_vector = translation_matrix.transform_vector(&self.position);
        // self.position = translation_vector;
    }

    fn rotate(&mut self, rotation: Vector3<f32>) {
        self.rotation += rotation;
    }

    fn scale(&mut self, scale: Vector3<f32>) {
        self.scale.component_mul_assign(&scale);
    }
}

fn matrix4_to_raw_array(matrix: &Matrix4<f32>) -> [[f32; 4]; 4] {
    let mut array = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            array[i][j] = matrix[(i, j)];
        }
    }
    array
}

// fn matrix4_to_raw_array(matrix: &nalgebra::Matrix4<f32>) -> [f32; 16] {
//     let mut raw_array = [0.0; 16];
//     for i in 0..4 {
//         for j in 0..4 {
//             raw_array[i * 4 + j] = matrix[(i, j)];
//         }
//     }
//     raw_array
// }

// Vertices for a pyramid
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // Apex
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // Base vertices
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        color: [1.0, 1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.0, 1.0, 1.0],
    },
];

// Indices for a pyramid
const INDICES: &[u16] = &[
    0, 1, 2, // Side 1
    0, 2, 3, // Side 2
    0, 3, 4, // Side 3
    0, 4, 1, // Side 4
    1, 3, 2, // Base 1
    1, 4, 3, // Base 2
];

static mut CAMERA: Option<SimpleCamera> = None;

thread_local! {
    static CAMERA_INIT: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn get_camera() -> &'static mut SimpleCamera {
    CAMERA_INIT.with(|init| {
        if !init.get() {
            unsafe {
                CAMERA = Some(SimpleCamera::new(
                    Point3::new(0.0, 0.0, 5.0),
                    Vector3::new(0.0, 0.0, -1.0),
                    Vector3::new(0.0, 1.0, 0.0),
                    45.0f32.to_radians(),
                    0.1,
                    100.0,
                ));
            }
            init.set(true);
        }
    });

    unsafe { CAMERA.as_mut().unwrap() }
}

struct RendererState {
    pyramids: Vec<Pyramid>,
    grids: Vec<Grid>,
    models: Vec<Model>,
}

impl RendererState {
    async fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        texture_render_mode_buffer: &wgpu::Buffer,
        color_render_mode_buffer: &wgpu::Buffer,
    ) -> Self {
        // create the utility grid(s)
        let mut grids = Vec::new();
        grids.push(Grid::new(
            &device,
            &model_bind_group_layout,
            color_render_mode_buffer,
        ));

        let mut pyramids = Vec::new();
        // pyramids.push(Pyramid::new(device, bind_group_layout, color_render_mode_buffer));
        // add more pyramids as needed

        // add the sample model
        let mut models = Vec::new();
        models.push(
            Model::from_glb(
                "http://localhost:1420/public/samples/replicate-prediction-cheeseburger.glb",
                device,
                queue,
                model_bind_group_layout,
                texture_bind_group_layout,
                texture_render_mode_buffer,
                color_render_mode_buffer,
            )
            .await,
        );

        Self {
            pyramids,
            grids,
            models,
        }
    }
}

static mut RENDERER_STATE: Option<RendererState> = None;

thread_local! {
    static RENDERER_STATE_INIT: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn get_renderer_state() -> &'static mut RendererState {
    RENDERER_STATE_INIT.with(|init| {
        if !init.get() {
            panic!("RendererState not initialized");
        }
    });

    unsafe { RENDERER_STATE.as_mut().unwrap() }
}

// native rendering loop
// #[wasm_bindgen]
// pub async fn init_wgpu() -> Result<(), JsValue> {
//     // pipeline setup...

//     event_loop
//         .run(move |event, target| {
//             if let Event::WindowEvent {
//                 window_id: _,
//                 event,
//             } = event
//             {
//                 match event {
//                     // WindowEvent::CursorMoved { position, .. } => {
//                     //     // Update the mouse position
//                     //     // println!("Mouse Position: {:?}", position);
//                     //     mouse_position = (position.x as f64, position.y as f64);
//                     // }
//                     // WindowEvent::MouseInput {
//                     //     state: ElementState::Pressed,
//                     //     button: MouseButton::Left,
//                     //     ..
//                     // } => {
//                     //     let window_size = (size.width as f64, size.height as f64);
//                     //     handle_click(window_size, mouse_position, &buttons, &labels);
//                     // }
//                     // TODO: handle mouse events for native purposes
//                     WindowEvent::Resized(new_size) => {
//                         // Reconfigure the surface with the new size
//                         // config.width = new_size.width.max(1);
//                         // config.height = new_size.height.max(1);
//                         // surface.configure(&device, &config);
//                         // On macos the window needs to be redrawn manually after resizing
//                         // window.request_redraw();
//                     }
//                     WindowEvent::RedrawRequested => {
//                         // necessary for native purposes? (not needed for web)
//                     }
//                     WindowEvent::CloseRequested => target.exit(),
//                     _ => {}
//                 };
//             }
//         })
//         .unwrap();

//     Ok(())
// }

// Your other imports...

#[wasm_bindgen]
pub async fn start_render_loop() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("scene-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Set focus on the canvas
    // canvas.focus().unwrap();

    // TODO: improve settings

    // Create logical components (instance, adapter, device, queue, surface, etc.)
    let dx12_compiler = wgpu::Dx12Compiler::Dxc {
        dxil_path: None, // Specify a path to custom location
        dxc_path: None,  // Specify a path to custom location
    };

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: dx12_compiler,
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Version2,
    });

    let height = canvas.height();
    let width = canvas.width();

    let event_loop = event_loop::EventLoop::new().unwrap();
    let builder = WindowBuilder::new().with_inner_size(LogicalSize::new(width, height));
    #[cfg(target_arch = "wasm32")] // necessary for web-sys
    let builder = {
        use winit::platform::web::WindowBuilderExtWebSys;
        builder.with_canvas(Some(canvas))
    };
    let winit_window = builder.build(&event_loop).unwrap();

    let surface = unsafe {
        instance
            .create_surface(winit_window)
            .expect("Couldn't create GPU Surface")
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or("Failed to find an appropriate adapter")
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // let swap_chain_format: wgpu::TextureFormat = surface.get_preferred_format(&adapter).unwrap();
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swap_chain_format = swapchain_capabilities.formats[0]; // Choosing the first available format

    // let size = canvas.get_bounding_client_rect();
    // let swap_chain_descriptor = wgpu::SurfaceConfiguration {
    //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //     format: swap_chain_format,
    //     width: width as u32,
    //     height: height as u32,
    //     present_mode: wgpu::PresentMode::Fifo,
    //     desired_maximum_frame_latency: 1,
    //     alpha_mode: wgpu::CompositeAlphaMode::Opaque,
    //     view_formats: vec![swap_chain_format], // Check?
    // };

    // surface.configure(&device, &swap_chain_descriptor);

    let mut config = surface.get_default_config(&adapter, width, height).unwrap();
    surface.configure(&device, &config);

    // Create the shader module
    let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/primary_vertex.wgsl").into()),
    });

    let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/primary_fragment.wgsl").into()),
    });

    let camera = get_camera();

    camera.update_aspect_ratio(config.width as f32 / config.height as f32);

    // hardcode position test
    // camera.position = Point3::new(0.0, 0.0, 10.0);

    // let usable_camera = camera.clone();
    // let usable_camera = usable_camera.borrow_mut();

    // Create the uniform buffer for the camera
    // let camera_matrix = camera.build_view_projection_matrix();
    camera.update_view_projection_matrix();
    let camera_matrix = camera.view_projection_matrix;
    let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Uniform Buffer"),
        contents: bytemuck::cast_slice(camera_matrix.as_slice()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Create the bind group for the uniform buffer
    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let model_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("model_bind_group_layout"),
        });

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Model Bind Group Layout"),
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_uniform_buffer.as_entire_binding(),
        }],
        label: Some("Bind Group"),
    });

    // create renderMode uniform for button backgrounds
    let color_render_mode_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Color Render Mode Buffer"),
        contents: bytemuck::cast_slice(&[0i32]), // Default to normal mode
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let color_render_mode_buffer = Arc::new(color_render_mode_buffer);

    // Create a buffer for the renderMode uniform
    let texture_render_mode_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Texture Render Mode Buffer"),
        contents: bytemuck::cast_slice(&[1i32]), // Default to text mode
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let texture_render_mode_buffer = Arc::new(texture_render_mode_buffer);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            &camera_bind_group_layout,
            &model_bind_group_layout,
            &texture_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth24Plus,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        label: Some("Depth Texture"),
        view_formats: &[],
    });

    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_stencil_state = wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth24Plus,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    };

    // Create the render pipeline
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "main",
            buffers: &[Vertex::desc()],
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: swap_chain_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        // depth_stencil: None,
        depth_stencil: Some(depth_stencil_state),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    // let context = generate_context!();
    // let package_info = context.package_info();
    // let env = context.env();

    // let resource_dir = resource_dir(&package_info, &env).unwrap();

    // web_sys::console::log_1(&format!("resource_dir: {:?}", resource_dir).into());

    let mut state = RendererState::new(
        &device,
        &queue,
        &model_bind_group_layout,
        &texture_bind_group_layout,
        &texture_render_mode_buffer,
        &color_render_mode_buffer,
    )
    .await;
    unsafe {
        RENDERER_STATE = Some(state);
        RENDERER_STATE_INIT.with(|init| {
            init.set(true);
        });
    }

    let state = get_renderer_state();

    // web-based rendering loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let closure = Closure::wrap(Box::new(move || {
        // Call your rendering function here
        render_frame(
            &state,
            &surface,
            &device,
            &queue,
            &render_pipeline,
            &depth_view,
            // &vertex_buffer,
            // &index_buffer,
            // &uniform_buffer,
            &camera_bind_group,
            // camera,
            &camera_uniform_buffer,
        );

        // Schedule the next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>);

    *g.borrow_mut() = Some(closure);

    // Start the rendering loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn render_frame(
    state: &RendererState,
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    render_pipeline: &wgpu::RenderPipeline,
    depth_view: &wgpu::TextureView,
    // vertex_buffer: &wgpu::Buffer,
    // index_buffer: &wgpu::Buffer,
    camera_bind_group: &wgpu::BindGroup,
    camera_uniform_buffer: &wgpu::Buffer,
    // camera: &mut SimpleCamera,
) {
    // draw frames...
    let mut camera = get_camera();

    // Render a frame
    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            // depth_stencil_attachment: None,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view, // This is the depth texture view
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0), // Clear to max depth
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None, // Set this if using stencil
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // draw calls...
        render_pass.set_pipeline(&render_pipeline);

        // In the render loop, update the uniform buffer if necessary
        // TODO: why uniform if updating every frame?
        camera.update();
        // web_sys::console::log_1(&format!("Camera position: {:?}", camera.position).into());
        let camera_matrix = camera.view_projection_matrix;
        queue.write_buffer(
            &camera_uniform_buffer,
            0,
            bytemuck::cast_slice(camera_matrix.as_slice()),
        );

        // draw utility grids
        // for grid in &state.grids {
        //     render_pass.set_bind_group(0, &camera_bind_group, &[]);
        //     render_pass.set_bind_group(1, &grid.bind_group, &[]);
        // TODO: texture_bind_group?

        //     render_pass.set_vertex_buffer(0, grid.vertex_buffer.slice(..));
        //     render_pass.set_index_buffer(grid.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        //     render_pass.draw_indexed(0..grid.index_count, 0, 0..1);
        // }

        // // draw pyramids
        // for pyramid in &state.pyramids {
        //     pyramid.update_uniform_buffer(&queue);
        //     render_pass.set_bind_group(0, &camera_bind_group, &[]);
        //     render_pass.set_bind_group(1, &pyramid.bind_group, &[]);

        //     render_pass.set_vertex_buffer(0, pyramid.vertex_buffer.slice(..));
        //     render_pass.set_index_buffer(pyramid.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        //     render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        // }

        for model in &state.models {
            for mesh in &model.meshes {
                render_pass.set_bind_group(0, &camera_bind_group, &[]);
                render_pass.set_bind_group(1, &mesh.bind_group, &[]);
                render_pass.set_bind_group(2, &mesh.texture_bind_group, &[]);

                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.draw_indexed(0..mesh.index_count as u32, 0, 0..1);
            }
        }
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
}

#[wasm_bindgen]
pub fn handle_key_press(key_code: String, is_pressed: bool) {
    let camera = get_camera();
    let state = get_renderer_state();

    println!("Key pressed: {}", key_code);
    web_sys::console::log_1(&format!("Key pressed (2): {}", key_code).into());

    match key_code.as_str() {
        "w" => {
            if is_pressed {
                // Handle the key press for W
                web_sys::console::log_1(&"Key W pressed".into());
                camera.position += camera.direction * 0.1;
            }
        }
        "s" => {
            if is_pressed {
                // Handle the key press for S
                web_sys::console::log_1(&"Key S pressed".into());
                camera.position -= camera.direction * 0.1;
            }
        }
        "a" => {
            if is_pressed {
                // Handle the key press for A
                web_sys::console::log_1(&"Key A pressed".into());
                let right = camera.direction.cross(&camera.up).normalize();
                camera.position -= right * 0.1;
            }
        }
        "d" => {
            if is_pressed {
                // Handle the key press for D
                web_sys::console::log_1(&"Key D pressed".into());
                let right = camera.direction.cross(&camera.up).normalize();
                camera.position += right * 0.1;
            }
        }
        "ArrowUp" => {
            if is_pressed {
                // Handle the key press for ArrowUp
                web_sys::console::log_1(&"Key ArrowUp pressed".into());
                state.pyramids[0].translate(Vector3::new(0.0, 0.1, 0.0));
                // test rotation
                // state.pyramids[0].rotate(Vector3::new(0.0, 0.1, 0.0));
                // test scale
                // state.pyramids[0].scale(Vector3::new(1.1, 1.1, 1.1));
            }
        }
        "ArrowDown" => {
            if is_pressed {
                // Handle the key press for ArrowDown
                web_sys::console::log_1(&"Key ArrowDown pressed".into());
                state.pyramids[0].translate(Vector3::new(0.0, -0.1, 0.0));
            }
        }
        "ArrowLeft" => {
            if is_pressed {
                // Handle the key press for ArrowLeft
                web_sys::console::log_1(&"Key ArrowLeft pressed".into());
                state.pyramids[0].translate(Vector3::new(-0.1, 0.0, 0.0));
            }
        }
        "ArrowRight" => {
            if is_pressed {
                // Handle the key press for ArrowRight
                web_sys::console::log_1(&"Key ArrowRight pressed".into());
                state.pyramids[0].translate(Vector3::new(0.1, 0.0, 0.0));
            }
        }
        _ => {
            // Handle any other keys if necessary
        }
    }

    camera.update();
}

#[wasm_bindgen]
pub fn handle_mouse_move(dx: f32, dy: f32) {
    let camera = get_camera();
    let sensitivity = 0.005;

    let dx = -dx * sensitivity;
    let dy = dy * sensitivity;

    camera.rotate(dx, dy);

    camera.update();
}
