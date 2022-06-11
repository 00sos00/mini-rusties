use glam::*;
use wgpu::{util::BufferInitDescriptor, util::DeviceExt, BufferUsages, Device, Queue};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const WORLD_UP: Vec3 = Vec3::Y;
const WORLD_RIGHT: Vec3 = Vec3::X;
const WORLD_FORWARD: Vec3 = Vec3::Z;

#[derive(Debug, PartialEq, Eq)]
enum TranslationType {
    Local,  // Move in the current camera direction
    Global, // Move relative to the world
}

struct Orientation {
    rotation: Quat,
}

impl Orientation {
    pub fn new() -> Self {
        Self { rotation: Quat::IDENTITY }
    }

    pub fn forward(&self) -> Vec3 {
        (self.rotation * -WORLD_FORWARD).normalize()
    }

    pub fn right(&self) -> Vec3 {
        let camera_forward = (self.rotation * -WORLD_FORWARD).normalize();
        
        camera_forward.cross(WORLD_UP).normalize()
    }

    pub fn up(&self) -> Vec3 {
        let camera_forward = (self.rotation * -WORLD_FORWARD).normalize();
        let camera_right = camera_forward.cross(WORLD_UP).normalize();

        camera_right.cross(camera_forward) // Or [self.rotation * WORLD_UP].normalize() for space-games
    }
}

pub struct Camera {
    current_translation: Vec3,
    new_translation: Vec3,
    orientation: Orientation,
    translation_type: TranslationType,
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
            current_translation: vec3(0.0, 0.0, 50.0),
            new_translation: vec3(0.0, 0.0, 100.0),
            orientation: Orientation::new(),
            translation_type: TranslationType::Local,
            aspect: 1920_f32 / 1080_f32,
            fov: 45.0,
            znear: 0.01,
            zfar: 100000.0,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn reset_translation(&mut self) {
        self.new_translation = vec3(0.0, 0.0, 100.0);
    }

    pub fn reset_orientation(&mut self) {
        self.orientation.rotation = Quat::IDENTITY;
    }

    pub fn rotate_around(&mut self, point: Vec3, axis: Vec3, angle: f32) {
        let offset = self.new_translation - point;
        let rotated_offset = Quat::from_axis_angle(axis, angle.to_radians()) * offset;

        self.new_translation = point + rotated_offset;
    }

    pub fn look_at(&mut self, point: Vec3) {
        let cam_forward = self.orientation.forward();
        let dir = (point - self.new_translation).normalize();

        let quat_w = (2.0 + 2.0 * cam_forward.dot(dir)).sqrt();
        let quat_vec = (1.0 / quat_w) * cam_forward.cross(dir);
        let quat =
            Quat::from_xyzw(quat_vec.x, quat_vec.y, quat_vec.z, 0.5 * quat_w).normalize();

        self.orientation.rotation = quat * self.orientation.rotation;
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32) {
        let camera_forward = self.orientation.forward();
        let camera_right = self.orientation.right();
        let camera_up = self.orientation.up();
        //let (cur_pitch, cur_yaw, cur_roll) = self.orientation.rotation.to_euler(EulerRot::XYZ);

        let x = Quat::from_axis_angle(camera_right, pitch.to_radians());
        let y = Quat::from_axis_angle(camera_up, -yaw.to_radians());
        let z = Quat::from_axis_angle(camera_forward, roll.to_radians());

        self.orientation.rotation = (z * y * x * self.orientation.rotation).normalize();
    }

    pub fn translate(&mut self, x_amount: f32, y_amount: f32, z_amount: f32) {
        let camera_forward = self.orientation.forward();
        let camera_right = self.orientation.right();
        let camera_up = self.orientation.up();

        if self.translation_type == TranslationType::Local {
            self.new_translation += camera_right * x_amount;
            self.new_translation += camera_up * y_amount;
            self.new_translation += camera_forward * z_amount;
        } else {
            self.new_translation += WORLD_RIGHT * x_amount;
            self.new_translation += WORLD_UP * y_amount;
            self.new_translation += -WORLD_FORWARD * z_amount;
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

        if mouse_offset != (0.0, 0.0) {
            let mouse_sensitivity = 10.0 * dt;

            let (mut mouse_offset_x, mut mouse_offset_y) = mouse_offset;
            mouse_offset_x *= mouse_sensitivity;
            mouse_offset_y *= -mouse_sensitivity; // We negate because the offset is inverted

            self.rotate(mouse_offset_x, mouse_offset_y, 0.0);
        }

        if input.key_held(VirtualKeyCode::Up) {
            self.rotate(0.0, move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::Down) {
            self.rotate(0.0, -move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::Right) {
            self.rotate(move_amount, 0.0, 0.0);
        }
        if input.key_held(VirtualKeyCode::Left) {
            self.rotate(-move_amount, 0.0, 0.0);
        }

        if input.key_held(VirtualKeyCode::E) {
            self.rotate(0.0, 0.0, roll_amount);
        }
        if input.key_held(VirtualKeyCode::Q) {
            self.rotate(0.0, 0.0, -roll_amount);
        }

        if input.key_pressed(VirtualKeyCode::G) {
            self.rotate_around(Vec3::ZERO, WORLD_UP, 45.0);
            self.look_at(Vec3::ZERO);
        }

        if input.key_pressed(VirtualKeyCode::T) {
            if self.translation_type == TranslationType::Local {
                self.translation_type = TranslationType::Global;
            } else {
                self.translation_type = TranslationType::Local;
            }
        }

        if input.key_held(VirtualKeyCode::W) {
            self.translate(0.0, 0.0, move_amount);
        }
        if input.key_held(VirtualKeyCode::S) {
            self.translate(0.0, 0.0, -move_amount);
        }
        if input.key_held(VirtualKeyCode::Space) {
            self.translate(0.0, move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::LShift) {
            self.translate(0.0, -move_amount, 0.0);
        }
        if input.key_held(VirtualKeyCode::D) {
            self.translate(move_amount, 0.0, 0.0);
        }
        if input.key_held(VirtualKeyCode::A) {
            self.translate(-move_amount, 0.0, 0.0);
        }

        if input.key_pressed(VirtualKeyCode::Tab) {
            self.reset_translation();
            self.reset_orientation();
        }
    }

    pub fn update_uniform_buffer(&mut self, queue: &Queue) {
        let uniform = self.uniform();

        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }

    fn uniform(&mut self) -> CameraUniform {
        self.current_translation = self.current_translation.lerp(self.new_translation, 0.15);

        let opengl_to_wgpu_matrix = Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 1.0),
        );

        let camera_forward = self.orientation.forward();
        let camera_up = self.orientation.up();

        let view = Mat4::look_at_rh(self.current_translation, self.current_translation + camera_forward, camera_up);
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
