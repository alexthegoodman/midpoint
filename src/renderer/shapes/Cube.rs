use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::util::DeviceExt;

use crate::renderer::{
    core::Vertex,
    Transform::{matrix4_to_raw_array, Transform},
};

// Vertices for a cube
const VERTICES: &[Vertex] = &[
    // Front face
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        tex_coords: [0.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        tex_coords: [1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        tex_coords: [1.0, 1.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        tex_coords: [0.0, 1.0],
        color: [1.0, 1.0, 0.0],
    },
    // Back face
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [1.0, 0.0],
        color: [1.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [1.0, 1.0],
        color: [0.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.0, 1.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.5, 0.5],
    },
];

// Indices for a cube
const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // Front face
    4, 5, 6, 6, 7, 4, // Back face
    3, 2, 6, 6, 5, 3, // Top face
    0, 4, 7, 7, 1, 0, // Bottom face
    1, 7, 6, 6, 2, 1, // Right face
    0, 3, 5, 5, 4, 0, // Left face
];

pub struct Cube {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    // uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub transform: Transform,
    pub index_count: u32,
}

impl Cube {
    pub fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let empty_buffer = Matrix4::<f32>::identity();
        let raw_matrix = matrix4_to_raw_array(&empty_buffer);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Uniform Buffer"),
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
            vertex_buffer,
            index_buffer,
            // uniform_buffer,
            bind_group,
            transform: Transform::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 1.0, 1.0),
                uniform_buffer,
            ),
            index_count: INDICES.len() as u32,
        }
    }
}
