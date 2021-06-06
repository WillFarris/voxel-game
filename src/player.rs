use cgmath::Vector3;

use crate::{block::{self, BLOCKS}, camera::Camera, collision::{self, rect_vs_rect}, vectormath::{Y_VECTOR, dda, len, normalize, normalize_inplace}, world::World};


const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.09, z: 0.0};

pub(crate) struct Player {
    pub camera: Camera,
    pub position: Vector3<f32>,
    move_speed: f32,
    pub direction: Vector3<f32>,
    grounded: bool,
    height: f32,
}

impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,
            move_speed: 5.0,
            direction: Vector3::new(0.0f32, 0.0f32, 0f32),
            grounded: false,
            height: 1.6,
        }
    }

    pub fn update(&mut self, world: &World, delta_time: f32) {

        if !BLOCKS[world.block_at_global_pos(Vector3::new(
            self.position.x.floor() as isize,
            self.position.y.floor() as isize - 1,
            self.position.z.floor() as isize))].solid {
            self.grounded = false;
        }


        let forward = normalize(&Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z));
        let delta = delta_time * Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * forward.x * self.direction.z as f32),
            y: self.move_speed * self.direction.y as f32,
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * forward.z * self.direction.z as f32),
        };


        let collision_box_dimensions = (0.25, 1.8);

        self.position.x += delta.x;
        let mut player_bounding_box = crate::collision::Rect3 {
            pos: Vector3::new(
                self.position.x - (collision_box_dimensions.0/2.0),
                self.position.y,
                self.position.z - (collision_box_dimensions.0/2.0)),
            size: Vector3::new(
                collision_box_dimensions.0,
                collision_box_dimensions.1,
                collision_box_dimensions.0
            )
        };
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = crate::collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let x_overlap = if player_bounding_box.pos.x > block_bounding_box.pos.x {
                            (block_bounding_box.pos.x + 1.0) - player_bounding_box.pos.x 
                        } else {
                            -1.0 * (player_bounding_box.pos.x + player_bounding_box.size.x - block_bounding_box.pos.x)
                        };
                        self.position.x += x_overlap;
                        player_bounding_box.pos.x += x_overlap;
                        println!("x: ({}, {}, {})", block_x, block_y, block_z);
                    }
                }
            }
        }

        self.position.y += delta.y;
        player_bounding_box = crate::collision::Rect3 {
            pos: Vector3::new(
                self.position.x - (collision_box_dimensions.0/2.0),
                self.position.y,
                self.position.z - (collision_box_dimensions.0/2.0)),
            size: Vector3::new(
                collision_box_dimensions.0,
                collision_box_dimensions.1,
                collision_box_dimensions.0
            )
        };
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = crate::collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let y_overlap = if player_bounding_box.pos.y > block_bounding_box.pos.y {
                            (block_bounding_box.pos.y + 1.0) - player_bounding_box.pos.y 
                        } else {
                            -1.0 * (player_bounding_box.pos.y + player_bounding_box.size.y - block_bounding_box.pos.y)
                        };

                        self.position.y += y_overlap;
                        player_bounding_box.pos.y += y_overlap;
                        println!("y: ({}, {}, {})", block_x, block_y, block_z);
                    }
                }
            }
        }

        self.position.z += delta.z;
        player_bounding_box = crate::collision::Rect3 {
            pos: Vector3::new(
                self.position.x - (collision_box_dimensions.0/2.0),
                self.position.y,
                self.position.z - (collision_box_dimensions.0/2.0)),
            size: Vector3::new(
                collision_box_dimensions.0,
                collision_box_dimensions.1,
                collision_box_dimensions.0
            )
        };
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = crate::collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let z_overlap = if player_bounding_box.pos.z > block_bounding_box.pos.z {
                            (block_bounding_box.pos.z + 1.0) - player_bounding_box.pos.z 
                        } else {
                            -1.0 * (player_bounding_box.pos.z + player_bounding_box.size.z - block_bounding_box.pos.z)
                        };
                        self.position.z += z_overlap;
                        player_bounding_box.pos.z += z_overlap;
                        println!("z: ({}, {}, {})", block_x, block_y, block_z);
                    }
                }
            }
        }

        self.camera.translate(self.position + self.height * Y_VECTOR);
    }

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.direction.x += direction.x;
        self.direction.z += direction.z;
        //if self.grounded {
            self.direction.y += direction.y;
            //self.grounded = false;
        //}
    }

    pub fn stop_move_direction(&mut self, direction: Vector3<f32>) {
        self.direction -= direction;
    }
}