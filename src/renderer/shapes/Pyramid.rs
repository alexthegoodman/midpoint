use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::util::DeviceExt;

use crate::renderer::core::Vertex;

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

pub struct Pyramid {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Pyramid {
    pub fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
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

    pub fn update_transform(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation =
            Matrix4::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        // web_sys::console::log_1(&format!("Pyramid translation: {:?}", translation).into());
        translation * rotation * scale
    }

    pub fn update_uniform_buffer(&self, queue: &wgpu::Queue) {
        let transform_matrix = self.update_transform();
        let transform_matrix = transform_matrix.transpose(); // Transpose to match wgpu layout
        let raw_matrix = matrix4_to_raw_array(&transform_matrix);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&raw_matrix));
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
        // web_sys::console::log_1(&format!("Pyramid position: {:?}", self.position).into());
        // alternative translation method
        // let translation_matrix = Matrix4::new_translation(&translation);
        // let translation_vector = translation_matrix.transform_vector(&self.position);
        // self.position = translation_vector;
    }

    pub fn rotate(&mut self, rotation: Vector3<f32>) {
        self.rotation += rotation;
    }

    pub fn scale(&mut self, scale: Vector3<f32>) {
        self.scale.component_mul_assign(&scale);
    }
}

pub fn matrix4_to_raw_array(matrix: &Matrix4<f32>) -> [[f32; 4]; 4] {
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
