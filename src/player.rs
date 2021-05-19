use cgmath::{Matrix3, Vector3, dot};

use crate::{block::BLOCKS, camera::Camera, vectormath::{X_VECTOR, Y_VECTOR, Z_VECTOR, dda, normalize, normalize_inplace}};

use std::cmp::min;

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.01, z: 0.0};

pub(crate) struct Player {
    pub camera: Camera,
    pub position: Vector3<f32>,
    move_speed: f32,
    direction: Vector3<f32>,
    grounded: bool,
}

impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,
            move_speed: 0.1,
            direction: Vector3::new(0.0f32, 0.0f32, 0f32),
            grounded: false,
        }
    }


    pub fn update(&mut self, chunk: &[[[usize; 16]; 16]; 16]  ) {
        if !self.grounded {
            self.direction.y -= 0.10;
        }

        let delta = Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * self.camera.forward.x * self.direction.z as f32),
            y: (self.move_speed * 0.5 * self.direction.y as f32),
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * self.camera.forward.z * self.direction.z as f32),
        };

        let cur_block = Vector3::new(
            self.position.x as isize,
            (self.position.y) as isize,
            self.position.z as isize,
        );

        if let Some((intersection, block)) = dda(chunk, &self.position, &delta) {
            println!("Collision at: {:?}", block);
        } else {
            self.position += delta;
        }

        self.camera.translate(self.position);
    }
    

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.direction.x += direction.x;
        self.direction.z += direction.z;
        if self.grounded {
            self.direction.y += direction.y;
        }
    }

    pub fn stop_move_direction(&mut self, direction: Vector3<f32>) {
        self.direction -= direction;
    }
}