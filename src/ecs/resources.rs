use glam::{Mat4, Vec3};
use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct Input {
    pub pressed_keys: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }
}

pub struct DeltaTime(pub f32);

impl Default for DeltaTime {
    fn default() -> Self {
        Self(0.0)
    }
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,

    yaw: f32,
    pitch: f32,

    view_matrix: Mat4,
    projection_matrix: Mat4,

    // Camera options
    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 3.0),
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),

            yaw: -90.0,
            pitch: 0.0,

            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        }
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn update_view_matrix(&mut self) {
        self.view_matrix = Mat4::look_at_rh(self.position, self.position + self.front, self.up);
    }

    pub fn update_projection_matrix(&mut self, aspect_ratio: f32) {
        self.projection_matrix =
            Mat4::perspective_rh_gl(self.zoom.to_radians(), aspect_ratio, 0.1, 100.0);
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match direction {
            CameraMovement::Forward => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left => self.position -= self.right * velocity,
            CameraMovement::Right => self.position += self.right * velocity,
        }
    }

    pub fn process_mouse_movement(
        &mut self,
        mut xoffset: f32,
        mut yoffset: f32,
        constrain_pitch: bool,
    ) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        self.zoom -= yoffset;
        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0;
        }
    }

    fn update_camera_vectors(&mut self) {
        let yaw_radians = self.yaw.to_radians();
        let pitch_radians = self.pitch.to_radians();

        let x = yaw_radians.cos() * pitch_radians.cos();
        let y = pitch_radians.sin();
        let z = yaw_radians.sin() * pitch_radians.cos();

        self.front = Vec3::new(x, y, z).normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}