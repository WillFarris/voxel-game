use cgmath::{Matrix3, Vector3, dot};

use crate::{block::BLOCKS, camera::Camera, vectormath::{X_VECTOR, Y_VECTOR, Z_VECTOR, normalize, normalize_inplace}};

use std::cmp::min;

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.01, z: 0.0};

#[derive(PartialEq)]
enum Vec3Direction {
    X,
    Y,
    Z
}


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
    (&self, chunk: &[[[usize; 16]; 16]; 16]) -> Option<Vector3<f32>> {
        let mut ray_start = Vector3 {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
        };

        let ray_dir = normalize(&self.camera.forward);

        let ray_unit_step_size = Vector3 {
            x: (1.0 + (ray_dir.y/ray_dir.x)*(ray_dir.y/ray_dir.x) + (ray_dir.z/ray_dir.x)*(ray_dir.z/ray_dir.x)).sqrt(),
            y: ((ray_dir.x/ray_dir.y)*(ray_dir.x/ray_dir.y) + 1.0 + (ray_dir.z/ray_dir.y)*(ray_dir.z/ray_dir.y)).sqrt(),
            z: ((ray_dir.x/ray_dir.z)*(ray_dir.x/ray_dir.z) + (ray_dir.y/ray_dir.z)*(ray_dir.y/ray_dir.z) + 1.0).sqrt(),
        };
        let mut map_check = Vector3 {
            x: ray_start.x.round() as i32,
            y: ray_start.y.round() as i32,
            z: ray_start.z.round() as i32
        };
        let mut ray_length_1d = Vector3 {x: 0.0, y: 0.0, z: 0.0 };
        let mut step = Vector3 {x: 0, y: 0, z: 0};

        if ray_dir.x < 0.0 {
            step.x = -1;
            ray_length_1d.x = (ray_start.x - map_check.x as f32) * ray_unit_step_size.x;
        } else {
            step.x = 1;
            ray_length_1d.x = ((map_check.x as f32 + 1.0) - ray_start.x) * ray_unit_step_size.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1;
            ray_length_1d.y = (ray_start.y - map_check.y as f32) * ray_unit_step_size.y;
        } else {
            step.y = 1;
            ray_length_1d.y = ((map_check.y as f32 + 1.0) - ray_start.y) * ray_unit_step_size.y;
        }

        if ray_dir.z < 0.0 {
            step.z = -1;
            ray_length_1d.z = (ray_start.z - map_check.z as f32) * ray_unit_step_size.z;
        } else {
            step.z = 1;
            ray_length_1d.z = ((map_check.z as f32 + 1.0) - ray_start.z) * ray_unit_step_size.z;
        }

        let mut tile_found = false;
        let max_dist = 4.0f32;
        let mut dist = 0.0;
        while !tile_found && (dist < max_dist) {

            let mut min_dist = ray_length_1d.x;
            let mut min_dir = Vec3Direction::X;
            if ray_length_1d.y < min_dist { min_dist = ray_length_1d.y; min_dir = Vec3Direction::Y }
            if ray_length_1d.z < min_dist { min_dist = ray_length_1d.z; min_dir = Vec3Direction::Z }

            if min_dir == Vec3Direction::X {
                map_check.x += step.x;
                dist = ray_length_1d.x;
                ray_length_1d.x += ray_unit_step_size.x;
            } else if min_dir == Vec3Direction::Y {
                map_check.y += step.y;
                dist = ray_length_1d.y;
                ray_length_1d.y += ray_unit_step_size.y;
            } else {
                map_check.z += step.z;
                dist = ray_length_1d.z;
                ray_length_1d.z += ray_unit_step_size.z;
            }
            if chunk[map_check.x as usize][map_check.y as usize][map_check.z as usize] != 0 {
                println!("Checked ({}, {}, {}). Hit!", map_check.x, map_check.y, map_check.z);
                tile_found = true;
                return Some(ray_start + ray_dir * dist);
            }
        }
        None
        /*let hit_radius = 40;
        let mut steps = 0;
        while steps < hit_radius {
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        if ray.x > (x as f32)-0.5 && ray.x < (x as f32)+0.5
                        && ray.y > (y as f32)-0.5 && ray.y < (y as f32)+0.5
                        && ray.z > (z as f32)-0.5 && ray.z < (z as f32)+0.5 {
                            println!("Chunk intersection with block at ({}, {}, {}): {}", x, y, z, chunk[x][y][z]);
                            if chunk[x][y][z] != 0 {
                                return Some(Vector3 {x, y, z});
                            }
                        }
                    }
                }
            }
            ray += 0.1 * self.camera.forward;
            steps += 1;
        }*/
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

        normalize_inplace(self.velocity);

    }

    pub fn stop_move_direction(&mut self, direction: Vector3<f32>) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.velocity.z = 0.0;
    }
}