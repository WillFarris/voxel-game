use cgmath::{Matrix3, Vector3, dot, num_traits::abs};

use crate::{block::BLOCKS, camera::Camera, player, vectormath::{self, X_VECTOR, Y_VECTOR, Z_VECTOR, dda, normalize, normalize_inplace}};

use std::cmp::min;

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.01, z: 0.0};

pub(crate) struct Player {
    pub camera: Camera,
    pub position: Vector3<f32>,
    move_speed: f32,
    pub direction: Vector3<f32>,
    pub delta: Vector3<f32>,
    grounded: bool,
}

impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,
            move_speed: 0.1,
            direction: Vector3::new(0.0f32, 0.0f32, 0f32),
            delta: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            grounded: false,
        }
    }

    pub fn update(&mut self, chunk: &[[[usize; 16]; 16]; 16]  ) {
        if !self.grounded {
            self.direction.y -= 0.1;
        }

        self.delta = Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * self.camera.forward.x * self.direction.z as f32),
            y: self.move_speed * 0.5 * self.direction.y as f32,
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * self.camera.forward.z * self.direction.z as f32),
        };
        let delta_mag = vectormath::len(&self.delta);
        if delta_mag >0.0 {println!("Delta: {:?}", self.delta);}

        
        if let Some((intersect, block, )) = dda(&chunk, &self.position, &self.delta, vectormath::len(&self.delta)) {
            self.delta = 0.9 * vectormath::len(&(intersect - self.position)) * normalize_inplace(self.delta);
            
            let player_block = Vector3::new(self.position.x as usize, self.position.y as usize, self.position.z as usize);
            if player_block.x as isize - block.x as isize == 0 {
                self.delta.z = 0.0;
            }
            if player_block.z as isize - block.z as isize == 0 {
                self.delta.x = 0.0;
            }
            if block.y < self.position.y as usize {
                self.grounded = true;
            }
            println!("Intersection: {:?}", intersect);
        }

        /*if let Some((dist, block)) = dda(chunk, &self.position, &delta) {
            println!("Detected nearby block: {:?}, dist: {}", block, dist);
            if dist < 1.0 {
                println!("{}", chunk[block.x][block.y][block.z]);
                if self.position.x as usize == block.x {

                }
            }
        }*/
        
        if chunk[self.position.x as usize][self.position.y as usize][self.position.z as usize] == 0 {   
            self.position += self.delta;
        }
        self.camera.translate(self.position);

        /*if !self.grounded {
            self.direction.y -= 0.10;
        }*/
    }

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.direction.x += direction.x;
        self.direction.z += direction.z;
        if self.grounded {
            self.direction.y += direction.y;
            self.grounded = false;
        }
    }

    pub fn stop_move_direction(&mut self, direction: Vector3<f32>) {
        self.direction -= direction;
    }
}