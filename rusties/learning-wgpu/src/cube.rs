use crate::{vertex::Vertex, transform::*};
use glam::*;
use wgpu::{util::BufferInitDescriptor, util::DeviceExt, BufferUsages, Device, Queue};

pub const CUBE_VERTICES: &[Vertex] = &[
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Front
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Vertex {
        // 1 Front top right
        position: [0.5, 0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 0 Front top left
        position: [-0.5, 0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 2 Front bottom left
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 2 Front bottom left
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 3 Front bottom right
        position: [0.5, -0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 1 Front top right
        position: [0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Back
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Vertex {
        // 4 Back top left
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 5 Back top right
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // 7 Back bottom right
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 7 Back bottom right
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 6 Back bottom left
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 4 Back top left
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Top
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Vertex {
        // 5 Back top right
        position: [0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 4 Back top left
        position: [-0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 0 Front top left
        position: [-0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // 0 Front top left
        position: [-0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // 1 Front top right
        position: [0.5, 0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 5 Back top right
        position: [0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Bottom
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Vertex {
        // 6 Back bottom left
        position: [-0.5, -0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 7 Back bottom right
        position: [0.5, -0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 3 Front bottom right
        position: [0.5, -0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 3 Front bottom right
        position: [0.5, -0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 2 Front bottom left
        position: [-0.5, -0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // 6 Back bottom left
        position: [-0.5, -0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Right
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Vertex {
        // 5 Back top right
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // 1 Front top right
        position: [0.5, 0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 3 Front bottom right
        position: [0.5, -0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 3 Front bottom right
        position: [0.5, -0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 7 Back bottom right
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 5 Back top right
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Left
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Vertex {
        // 0 Front top left
        position: [-0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // 4 Back top left
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // 6 Back bottom left
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 6 Back bottom left
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // 2 Front bottom left
        position: [-0.5, -0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // 0 Front top left
        position: [-0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
];

/* pub const CUBE_INDICES: &[u16] = &[
    1, 0, 2,    2, 3, 1, // Front face
    4, 5, 7,    7, 6, 4, // Back face *
    5, 4, 0,    0, 1, 5, // Top face
    6, 7, 3,    3, 2, 6, // Bottom face *
    5, 1, 3,    3, 7, 5, // Right face
    0, 4, 6,    6, 2, 0, // left face *
]; */

pub struct Cube {
    pub transform: Transform,
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Cube {
    pub fn new(device: &Device, label: &'static str) -> Self {
        let uniform = CubeUniform {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("{label} Buffer")),
            contents: bytemuck::bytes_of(&uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some(&format!("{label} Bind Group Layout")),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some(&format!("{label} Bind Group")),
        });

        Self {
            transform: Transform::default(),
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    fn model_mat(&self) -> Mat4 {
        let scale_matrix = Mat4::from_scale(self.transform.scale);
        let rotation_matrix = Mat4::from_euler(
            EulerRot::XYZ,
            self.transform.orientation.pitch.to_radians(),
            self.transform.orientation.yaw.to_radians(),
            self.transform.orientation.roll.to_radians(),
        );
        let translation_matrix = Mat4::from_translation(self.transform.translation);

        translation_matrix * rotation_matrix * scale_matrix
    }

    pub fn update_uniform_buffer(&self, queue: &Queue) {
        let model = self.model_mat();
        let uniform = CubeUniform {
            model_matrix: model.to_cols_array_2d(),
        };

        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }
}

type Mat4x4 = [[f32; 4]; 4];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubeUniform {
    model_matrix: Mat4x4,
}
