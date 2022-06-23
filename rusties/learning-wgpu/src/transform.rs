#![allow(dead_code)]
use glam::*;

// lEFT HANDED COORDINATE SYSTEM
pub const WORLD_RIGHT: Vec3 = Vec3::X;
pub const WORLD_UP: Vec3 = Vec3::Y;
pub const WORLD_FORWARD: Vec3 = Vec3::NEG_Z;

fn absolute_angle(a: f32) -> f32 {
    let ar = a % 360.0;

    if ar == 0.0 {
        return 0.0;
    }

    if ar > 0.0 && ar < 180.0 {
        ar
    } else if ar > 0.0 && ar > 180.0 {
        -(180.0 - ar % 180.0)
    } else if ar < 0.0 && ar > -180.0 {
        ar
    } else {
        180.0 + ar % 180.0
    }
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
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub rotation: Quat,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation {
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            forward: WORLD_FORWARD,
            right: WORLD_RIGHT,
            up: WORLD_UP,
            rotation: Quat::IDENTITY,
        }
    }
}

impl Orientation {
    pub fn right(&self) -> Vec3 {
        let self_forward = self.forward();

        self_forward.cross(WORLD_UP).normalize()
    }

    pub fn up(&self) -> Vec3 {
        let self_forward = self.forward();
        let self_right = self_forward.cross(WORLD_UP).normalize();

        self_right.cross(self_forward) // Or [self.rotation * WORLD_UP].normalize() for space-games
    }

    pub fn forward(&self) -> Vec3 {
        (self.rotation * WORLD_FORWARD).normalize()
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
        let self_right = self.orientation.right;
        let self_up = self.orientation.up;
        let self_forward = self.orientation.forward;

        self.translation += if self.transform_state == TransformState::Local {
            self_right * x + self_up * y + self_forward * z
        } else {
            WORLD_RIGHT * x + WORLD_UP * y + WORLD_FORWARD * z
        };
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale += WORLD_RIGHT.abs() * x + WORLD_UP.abs() * y + WORLD_FORWARD.abs() * z;
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.orientation.pitch += pitch; //absolute_angle(self.orientation.pitch + pitch);
        self.orientation.yaw -= yaw; //absolute_angle(self.orientation.yaw - yaw);
        self.orientation.roll += roll; //absolute_angle(self.orientation.roll + roll);

        let (right, up, forward) = if self.transform_state == TransformState::Local {
            (
                self.orientation.right,
                self.orientation.up,
                self.orientation.forward,
            )
        } else {
            (WORLD_RIGHT, WORLD_UP, WORLD_FORWARD)
        };

        let x = Quat::from_axis_angle(right, pitch.to_radians());
        let y = Quat::from_axis_angle(WORLD_UP, -yaw.to_radians());
        let z = Quat::from_axis_angle(forward, roll.to_radians());
        let quat = (x * y * z).normalize();
        /* let quat = Quat::from_euler(
            EulerRot::ZYX,
            self.orientation.roll.to_radians(),
            self.orientation.yaw.to_radians(),
            self.orientation.pitch.to_radians(),
        ); */

        self.orientation.rotation = (x * y * z * self.orientation.rotation).normalize();
        self.orientation.forward = self.orientation.forward();
        self.orientation.right = self.orientation.right();
        self.orientation.up = self.orientation.up();
    }

    /// Rotates the transform around a certain point in 3D space with the specified axis and angle (in degrees)
    pub fn rotate_around(&mut self, point: Vec3, axis: Vec3, angle: f32) {
        let offset = self.translation - point;
        let rotated_offset = Quat::from_axis_angle(axis, angle.to_radians()) * point;

        self.translation = offset + rotated_offset;
    }

    pub fn look_at(&mut self, point: Vec3) {
        if point == self.translation {
            return;
        }

        let self_forward = self.orientation.forward;
        let dir = (point - self.translation).normalize();

        let quat_w = (2.0 + 2.0 * self_forward.dot(dir)).sqrt();
        let quat_vec = (1.0 / quat_w) * self_forward.cross(dir);
        let quat = Quat::from_xyzw(quat_vec.x, quat_vec.y, quat_vec.z, 0.5 * quat_w).normalize();
        let quat2 = Quat::from_rotation_arc_colinear(self_forward, dir).normalize();

        //self.orientation.rotation = (quat2 * self.orientation.rotation).normalize();
        self.orientation.forward = quat * self.orientation.forward;
        self.orientation.right = quat * self.orientation.right;
        self.orientation.up = quat * self.orientation.up;
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
