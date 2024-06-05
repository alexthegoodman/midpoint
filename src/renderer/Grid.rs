use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::util::DeviceExt;

use crate::renderer::core::Vertex;
use crate::renderer::shapes::Pyramid::matrix4_to_raw_array;

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
