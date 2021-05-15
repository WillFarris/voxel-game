use cgmath::{Vector3, dot};

use crate::{block::BLOCKS, camera::Camera, vectormath::{X_VECTOR, Y_VECTOR, Z_VECTOR, normalize, normalize_inplace}};


const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.01, z: 0.0};


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
            move_speed: 0.1,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            grounded: false,
        }
    }

    pub fn collisions(&mut self, chunk: &[[[usize; 16]; 16]; 16]) {
        let cur_block = Vector3::new(
            self.position.x.round() as isize,
            (self.position.y-1.5).round() as isize,
            self.position.z.round() as isize,
        );
        if  cur_block.x >= 0 && cur_block.x < 15 &&
            cur_block.y >= 0 && cur_block.x < 15 && 
            cur_block.y >= 0 && cur_block.x < 15 {
            if chunk[cur_block.x as usize][(cur_block.y) as usize][cur_block.z as usize] != 0 {
                self.velocity.y = 0.0;
                self.grounded = true;
            } else {
                self.grounded = false;
            }
        }
    }

    pub fn kinematics(&mut self) {
        if self.grounded {
            self.velocity.y  = 0.0;
        } else {
            self.velocity += GRAVITY;
        }
        self.position += self.move_speed * self.velocity;

        self.camera.translate(self.position);
    }

    pub fn dda
    (&self, chunk: &[[[usize; 16]; 16]; 16]) -> Option<Vector3<usize>> {
        let mut ray_dir = normalize_inplace(self.camera.forward - self.position);



        /*let hit_radius = 20;
        let mut steps = 0;
        while steps < hit_radius {
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        if ray.0 > (x as f32)-0.5 && ray.0 < (x as f32)+0.5
                        && ray.1 > (y as f32)-0.5 && ray.1 < (y as f32)+0.5
                        && ray.2 > (z as f32)-0.5 && ray.2 < (z as f32)+0.5 {
                            println!("Chunk intersection with block at ({}, {}, {}): {}", x, y, z, chunk[x][y][z]);
                            if chunk[x][y][z] != 0 {
                                return Some(Vector3 {x, y, z});
                            }
                        }
                    }
                }
            }
            ray.0 += 0.25 * self.camera.forward[0];
            ray.1 += 0.25 * self.camera.forward[1];
            ray.2 += 0.25 * self.camera.forward[2];
            steps += 1;
        }*/


        None
    }
    

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.velocity.x += self.camera.right.x * direction.x;
        self.velocity.y += self.camera.right.y * direction.x;
        self.velocity.z += self.camera.right.z * direction.x;

        if direction.y > 0.0 {
            self.velocity.y += self.camera.up.y * direction.y;
            self.grounded = false;
        }

        self.velocity.x += self.camera.forward.x * direction.z;
        self.velocity.y += self.camera.forward.y * direction.z;
        self.velocity.z += self.camera.forward.z * direction.z;

    }

    pub fn stop_move_direction(&mut self, direction: Vector3<f32>) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.velocity.z = 0.0;
    }
}