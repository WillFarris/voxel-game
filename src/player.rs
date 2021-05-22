use cgmath::{Matrix3, Vector3, Vector4, dot, num_traits::abs};

use crate::{block::BLOCKS, camera::Camera, player, vectormath::{self, X_VECTOR, Y_VECTOR, Z_VECTOR, dda, normalize, len, normalize_inplace}};

use std::cmp::max;

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.04, z: 0.0};

pub(crate) struct Player {
    pub camera: Camera,
    pub position: Vector3<f32>,
    move_speed: f32,
    pub direction: Vector3<f32>,
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

        let forward = normalize_inplace(Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z));
        let right = self.camera.right;

        let mut delta = Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * forward.x * self.direction.z as f32),
            y: self.move_speed * 0.5 * self.direction.y as f32,
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * forward.z * self.direction.z as f32),
        };
        self.direction.y /= 1.01;

        let mut collision_box = Vec::with_capacity(4);
        collision_box.push((0.25 * right) + (0.25 * forward) + delta);
        collision_box.push((-0.25 * right) + (0.25 * forward) + delta);
        collision_box.push((0.25 * right) - (0.25 * forward) + delta);
        collision_box.push((-0.25 * right) - (0.25 * forward) + delta);

        let mut collision = false;
        for vert in collision_box {
            if chunk[(self.position.x + delta.x + vert.x) as usize][(self.position.y + delta.y + vert.y) as usize][(self.position.z + delta.z + vert.z) as usize] != 0 ||
               chunk[(self.position.x + delta.x + vert.x) as usize][(self.position.y + delta.y + vert.y + 1.0) as usize][(self.position.z + delta.z + vert.z) as usize] != 0 {
                if let Some((intersect, block)) = dda(&chunk, &(self.position + delta), &vert, len(&(vert))) {
                    println!("Collision at {:?} {} from player feet", intersect, len(&(intersect - self.position)));
                    //intersect - (vert + self.position);
                    if (self.position.x + delta.x) as usize == block.x {
                        delta.z = 0.0;
                    }
                    if (self.position.z + delta.z) as usize == block.z {
                        delta.x = 0.0;
                    }

                    
                    //delta -= vert;
                    collision = true;
                }
                if let Some((intersect, block)) = dda(&chunk, &(self.position + delta + Y_VECTOR), &vert, len(&(vert))) {
                    println!("Collision at {:?} {} from player head", intersect, len(&(intersect - self.position)));
                    //intersect - (vert + self.position);
                    if (self.position.x + delta.x) as usize == block.x {
                        delta.z = 0.0;
                    }
                    if (self.position.z + delta.z) as usize == block.z {
                        delta.x = 0.0;
                    }

                    
                    //delta -= vert;
                    collision = true;
                }
            }
        }

        if chunk[self.position.x as usize%16][(self.position.y-0.1) as usize%16][self.position.z as usize%16] != 0 {
            //println!("Standing on a block");
            self.grounded = true;
            //self.direction.y = 0.0;
        } else {
            self.grounded = false;
            delta += GRAVITY;
        }

        let delta_mag = vectormath::len(&delta);
        //if delta_mag > 0.0 {println!("Delta: {:?} length {}", delta, delta_mag);}
        
        /*if let Some((intersect, block)) = dda(&chunk, &self.position, &delta, delta_mag) {
            let mut intersect_direction = intersect - self.position;
            let intersect_dist = len(&intersect_direction);

            

            if block.y < self.position.y as usize {
                self.direction.y = 0.0;
                self.grounded = true;
            }
            if block.x == self.position.x as usize {
                intersect_direction.z = 0.0;
            }
            if block.z == self.position.z as usize {
                intersect_direction.x = 0.0;
            }
            //let mut normalized_delta = normalize(&delta);

            if intersect_dist > delta_mag {
                self.position += delta;
            } else {
                self.position.x += 0.9 * intersect_direction.x;// - (0.02 * normalized_delta.x);
                self.position.z += 0.9 * intersect_direction.z;// - (0.02 * normalized_delta.z);
            }
        } else {
            self.position += delta;
        }*/
        //if !collision {self.position += delta;}
        self.position += delta;
        self.camera.translate(self.position + 2.0 * Y_VECTOR);
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