use cgmath::Vector3;

use crate::{camera::Camera, vectormath::{Y_VECTOR, cross, normalize, normalize_inplace}};

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: 0.0, z: 0.0};


pub(crate) struct Player {
    pub camera: Camera,
    pub position: Vector3<f32>,
    move_speed: f32,
    velocity: Vector3<f32>,
    grounded: bool,
}


impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,
            move_speed: 1.0,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            grounded: false,
        }
    }

    pub fn update(&mut self) {
        if self.grounded {
            self.velocity.y = 0.0;
        }
        self.position += self.move_speed * self.velocity;        

        self.camera.translate(self.position);
    }

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.velocity.x = self.camera.right.x * direction.x;
        self.velocity.z = self.camera.right.z * direction.z;

        self.velocity.x = self.camera.forward.x * direction.x;
        self.velocity.z = self.camera.forward.z * direction.z;

    }

    pub fn stop_move_direction(&mut self, direction: Vector3<f32>) {
        self.velocity.x = self.camera.right.x * direction.x;
        self.velocity.z = self.camera.right.z * direction.x;

        self.velocity.x = self.camera.forward.x * direction.z;
        self.velocity.z = self.camera.forward.z * direction.z;
    }
}