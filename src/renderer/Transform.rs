use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::util::DeviceExt;

use crate::renderer::core::Vertex;

pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    uniform_buffer: wgpu::Buffer,
}

impl Transform {
    pub fn new(
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: Vector3<f32>,
        uniform_buffer: wgpu::Buffer,
    ) -> Self {
        Self {
            position,
            rotation,
            scale,
            uniform_buffer,
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
