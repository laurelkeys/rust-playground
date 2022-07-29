use cgmath::{InnerSpace, Matrix4, Point3, Vector3};
use winit::event::*;

//
// Camera
//

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    /// Camera aspect ratio.
    pub aspect: f32,
    /// Vertical field of view.
    pub y_fov: cgmath::Deg<f32>,
    /// Near clipping distance.
    pub z_near: f32,
    /// Far clipping distance.
    pub z_far: f32,
}

impl Camera {
    /// Returns a matrix that transforms world coordinates to clip coordinates, e.g.:
    /// ```
    /// let world_point = ...
    /// let clip_from_world = Camera::build_view_projection_matrix();
    /// let clip_point = clip_from_world * world_point; // projection
    /// ```
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_from_world = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let clip_from_view = cgmath::perspective(self.y_fov, self.aspect, self.z_near, self.z_far);
        clip_from_view * view_from_world
    }
}

//
// CameraController
//

// @Todo: replace this with https://github.com/h3r2tic/dolly
// (then, it'd also make sense to replace cgmath with glam).
pub struct CameraController {
    speed: f32,
    is_pressed: IsPressed,
}

use bitflags::bitflags;

bitflags! {
    #[derive(Default)] // empty()
    struct IsPressed: u32 {
        const UP       = 0b000001;
        const DOWN     = 0b000010;
        const LEFT     = 0b000100;
        const RIGHT    = 0b001000;
        const FORWARD  = 0b010000;
        const BACKWARD = 0b100000;
    }
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed, is_pressed: IsPressed::default() }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput { state, virtual_keycode: Some(keycode), .. },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_pressed.set(IsPressed::UP, is_pressed);
                    }
                    VirtualKeyCode::LShift => {
                        self.is_pressed.set(IsPressed::DOWN, is_pressed);
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_pressed.set(IsPressed::LEFT, is_pressed);
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_pressed.set(IsPressed::RIGHT, is_pressed);
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_pressed.set(IsPressed::FORWARD, is_pressed);
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_pressed.set(IsPressed::BACKWARD, is_pressed);
                    }
                    _ => return false,
                }
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        let forward = forward / forward_mag; // forward.normalize()

        if self.is_pressed.contains(IsPressed::FORWARD) {
            if forward_mag > self.speed {
                camera.eye += forward * self.speed;
            } else {
                // Do nothing.
            }
        }
        if self.is_pressed.contains(IsPressed::BACKWARD) {
            camera.eye -= forward * self.speed;
        }

        let right = forward.cross(camera.up);
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_pressed.contains(IsPressed::LEFT) {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
        if self.is_pressed.contains(IsPressed::RIGHT) {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
    }
}
