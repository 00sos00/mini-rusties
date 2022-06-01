use cgmath::*;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            eye: (0.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1920_f32 / 1080_f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn process_input(&mut self, input: &WinitInputHelper) {
        if input.key_held(VirtualKeyCode::A) {
            // not sure
        }
    }

    pub fn uniform(&self) -> CameraUniform {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        let view_proj_matrix = OPENGL_TO_WGPU_MATRIX * proj * view;

        CameraUniform {
            view_proj_matrix: view_proj_matrix.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj_matrix: [[f32; 4]; 4],
}
