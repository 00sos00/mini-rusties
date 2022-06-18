use crate::transform::*;
use glam::*;
use wgpu::{util::BufferInitDescriptor, util::DeviceExt, BufferUsages, Device, Queue};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Camera {
    pub transform: Transform,
    pub aspect: f32,
    pub fov: f32,
    znear: f32,
    zfar: f32,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new(device: &Device, label: &'static str) -> Self {
        let uniform = CameraUniform {
            view_proj_matrix: Mat4::IDENTITY.to_cols_array_2d(),
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
            aspect: 1920_f32 / 1080_f32,
            fov: 45.0,
            znear: 0.01,
            zfar: 100000.0,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn process_input(
        &mut self,
        mouse_offset: (f32, f32),
        input: &WinitInputHelper,
        _time_since_start: f32,
        dt: f32,
    ) {
        let move_amount = 100.0 * dt;
        let roll_amount = 100.0 * dt;
        let mouse_sensitivity = 10.0 * dt;

        if input.key_pressed(VirtualKeyCode::T) {
            if self.transform.transform_state == TransformState::Local {
                self.transform.transform_state = TransformState::Global;
            } else {
                self.transform.transform_state = TransformState::Local;
            }
        }

        if input.key_held(VirtualKeyCode::W) {
            self.transform.translate(0.0, 0.0, move_amount);
        }
        if input.key_held(VirtualKeyCode::S) {
            self.transform.translate(0.0, 0.0, -move_amount);
        }
        if input.key_held(VirtualKeyCode::Space) {
            self.transform.translate(0.0, move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::LShift) {
            self.transform.translate(0.0, -move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::D) {
            self.transform.translate(move_amount, 0.0, 0.0);
        }
        if input.key_held(VirtualKeyCode::A) {
            self.transform.translate(-move_amount, 0.0, 0.0);
        }

        if input.key_pressed(VirtualKeyCode::Tab) {
            self.transform.translation = Vec3::ZERO;
            self.transform.orientation.reset();
        }

        if mouse_offset != (0.0, 0.0) {
            let (mut mouse_offset_x, mut mouse_offset_y) = mouse_offset;
            mouse_offset_x *= mouse_sensitivity;
            mouse_offset_y *= -mouse_sensitivity; // We negate because the offset is inverted

            self.transform.rotate(mouse_offset_y, mouse_offset_x, 0.0);
        }

        if input.key_held(VirtualKeyCode::Up) {
            self.transform.rotate(move_amount, 0.0, 0.0);
        }
        if input.key_held(VirtualKeyCode::Down) {
            self.transform.rotate(-move_amount, 0.0, 0.0);
        }
        if input.key_held(VirtualKeyCode::Right) {
            self.transform.rotate(0.0, move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::Left) {
            self.transform.rotate(0.0, -move_amount, 0.0);
        }

        if input.key_held(VirtualKeyCode::E) {
            self.transform.rotate(0.0, 0.0, roll_amount);
        }
        if input.key_held(VirtualKeyCode::Q) {
            self.transform.rotate(0.0, 0.0, -roll_amount);
        }

        if input.key_pressed(VirtualKeyCode::G) {
            self.transform
                .rotate_around(vec3(0.0, 0.0, -100.0), *WORLD_UP, 45.0);
            //self.transform.look_at(vec3(0.0, 0.0, -100.0));
        }
    }

    pub fn update_uniform_buffer(&mut self, queue: &Queue) {
        let uniform = self.uniform();

        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }

    fn uniform(&mut self) -> CameraUniform {
        //self.current_translation = self.current_translation.lerp(self.new_translation, 0.15);

        /* let f = self.transform.orientation.forward;
        let u = self.transform.orientation.up;
        let fyz = vec3(0.0, f.y, f.z);
        let fxz = vec3(f.x, 0.0, f.z);
        let uxy = vec3(u.x, u.y, 0.0);

        let pitch = fyz.dot(*WORLD_FORWARD);
        let yaw = fxz.dot(*WORLD_FORWARD);
        let roll = uxy.dot(*WORLD_UP);

        println!("{pitch:?} {yaw:?} {roll:?}"); */

        let opengl_to_wgpu_matrix = Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 1.0),
        );

        let camera_forward = self.transform.orientation.forward();
        let camera_up = self.transform.orientation.up();

        let view = Mat4::look_at_rh(self.transform.translation, self.transform.translation + camera_forward, camera_up);
        let proj = Mat4::perspective_rh(self.fov.to_radians(), self.aspect, self.znear, self.zfar);

        CameraUniform {
            view_proj_matrix: (opengl_to_wgpu_matrix * proj * view).to_cols_array_2d(),
        }
    }
}

type Mat4x4 = [[f32; 4]; 4];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj_matrix: Mat4x4,
}
