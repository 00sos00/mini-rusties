#![allow(dead_code)]
use glam::*;

lazy_static::lazy_static! {
    // Left handed
    pub static ref WORLD_RIGHT: Vec3 = Vec3::X;
    pub static ref WORLD_UP: Vec3 = Vec3::Y;
    pub static ref WORLD_FORWARD: Vec3 = -Vec3::Z;
}

#[derive(Debug, PartialEq, Eq)]
pub enum TransformState {
    Local,
    Global,
}

#[derive(Debug)]
pub struct Orientation {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub rotation: Quat,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation {
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            rotation: Quat::IDENTITY,
        }
    }
}

impl Orientation {
    pub fn right(&self) -> Vec3 {
        let self_forward = (self.rotation * *WORLD_FORWARD).normalize();

        self_forward.cross(*WORLD_UP).normalize()
    }

    pub fn up(&self) -> Vec3 {
        let self_forward = (self.rotation * *WORLD_FORWARD).normalize();
        let self_right = self_forward.cross(*WORLD_UP).normalize();

        self_right.cross(self_forward) // Or [self.rotation * *WORLD_UP].normalize() for space-games
    }

    pub fn forward(&self) -> Vec3 {
        (self.rotation * *WORLD_FORWARD).normalize()
    }

    pub fn reset(&mut self) {
        self.pitch = 0.0;
        self.yaw = 0.0;
        self.roll = 0.0;
        self.rotation = Quat::IDENTITY;
    }
}

#[derive(Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub orientation: Orientation,
    pub scale: Vec3,
    pub transform_state: TransformState,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            orientation: Orientation::default(),
            scale: Vec3::ONE,
            transform_state: TransformState::Local,
        }
    }
}

impl Transform {
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let self_right = self.orientation.right();
        let self_up = self.orientation.up();
        let self_forward = self.orientation.forward();

        if self.transform_state == TransformState::Local {
            self.translation += self_right * x;
            self.translation += self_up * y;
            self.translation += self_forward * z;
        } else {
            self.translation += *WORLD_RIGHT * x;
            self.translation += *WORLD_UP * y;
            self.translation += *WORLD_FORWARD * z;
        }
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let self_right = self.orientation.right();
        let self_up = self.orientation.up();
        let self_forward = self.orientation.forward();

        if self.transform_state == TransformState::Local {
            self.scale += self_right.abs() * x;
            self.scale += self_up.abs() * y;
            self.scale += self_forward.abs() * z;
        } else {
            self.scale += WORLD_RIGHT.abs() * x;
            self.scale += WORLD_UP.abs() * y;
            self.scale += WORLD_FORWARD.abs() * z;
        }
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.orientation.pitch += pitch;
        self.orientation.yaw += yaw;
        self.orientation.roll += roll;

        let (right, up, forward) = if self.transform_state == TransformState::Local {
            (
                self.orientation.right(),
                self.orientation.up(),
                self.orientation.forward(),
            )
        } else {
            (*WORLD_RIGHT, *WORLD_UP, -*WORLD_FORWARD)
        };

        let x = Quat::from_axis_angle(right, pitch.to_radians());
        let y = Quat::from_axis_angle(*WORLD_UP, -yaw.to_radians());
        let z = Quat::from_axis_angle(forward, roll.to_radians());

        self.orientation.rotation = (x * y * z * self.orientation.rotation).normalize();
    }

    /// Rotates the transform around a certain point in 3D space with the specified axis and angle (in degrees)
    pub fn rotate_around(&mut self, point: Vec3, axis: Vec3, angle: f32) {
        let offset = self.translation - point;
        let rotated_offset = Quat::from_axis_angle(axis, angle.to_radians()) * offset;

        self.translation = point + rotated_offset;
    }

    pub fn look_at(&mut self, point: Vec3) {
        let self_forward = self.orientation.forward();
        let dir = (point - self.translation).normalize();

        let quat_w = (2.0 + 2.0 * self_forward.dot(dir)).sqrt();
        let quat_vec = (1.0 / quat_w) * self_forward.cross(dir);
        let quat = Quat::from_xyzw(quat_vec.x, quat_vec.y, quat_vec.z, 0.5 * quat_w).normalize();

        self.orientation.rotation = (quat * self.orientation.rotation).normalize();
        /* let (pitch, yaw, roll) = self.orientation.rotation.to_euler(EulerRot::XYZ);

        (
            self.orientation.pitch,
            self.orientation.yaw,
            self.orientation.roll,
        ) = (pitch, yaw, roll); */
    }

    pub fn lerp(&mut self, other: Self, t: f32) {
        self.translation = self.translation.lerp(other.translation, t);
        self.scale = self.scale.lerp(other.scale, t);
        self.orientation.rotation = self
            .orientation
            .rotation
            .lerp(other.orientation.rotation, t);

        let (pitch, yaw, roll) = self.orientation.rotation.to_euler(EulerRot::XYZ);

        (
            self.orientation.pitch,
            self.orientation.yaw,
            self.orientation.roll,
        ) = (pitch, yaw, roll);
    }

    pub fn lerp_translation(&mut self, other: Self, t: f32) {
        self.translation = self.translation.lerp(other.translation, t);
    }

    pub fn lerp_scale(&mut self, other: Self, t: f32) {
        self.scale = self.scale.lerp(other.scale, t);
    }

    pub fn lerp_rotation(&mut self, other: Self, t: f32) {
        self.orientation.rotation = self
            .orientation
            .rotation
            .lerp(other.orientation.rotation, t);

        let (pitch, yaw, roll) = self.orientation.rotation.to_euler(EulerRot::XYZ);

        (
            self.orientation.pitch,
            self.orientation.yaw,
            self.orientation.roll,
        ) = (pitch, yaw, roll);
    }
}
