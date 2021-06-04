use cgmath::Vector3;

use crate::{block::BLOCKS, camera::Camera, collision, vectormath::{Y_VECTOR, dda, len, normalize, normalize_inplace}, world::World};


const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -0.05, z: 0.0};

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

        let radius = 0.4;

        let forward = self.camera.forward;
        let delta = delta_time * Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * forward.x * self.direction.z as f32),
            y: self.move_speed * self.direction.y as f32,
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * forward.z * self.direction.z as f32),
        };

        /*let mut potential_position = self.position + delta;
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 1) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }

                    let nearest_point = Vector3 {
                        x: (block_x as f32).max(potential_position.x.min(block_x as f32 + 1.0)),
                        y: (block_y as f32).max(potential_position.y.min(block_y as f32 + 1.0)),
                        z: (block_z as f32).max(potential_position.z.min(block_z as f32 + 1.0)),
                    };

                    let ray_to_nearest = nearest_point - potential_position;
                    let mut overlap = radius - len(&ray_to_nearest);
                    if overlap.is_nan() { overlap = 0.0; }

                    if overlap > 0.0 {
                        potential_position = potential_position - normalize(&ray_to_nearest) * overlap;
                    }
                }
            }   
        }
        self.position = potential_position;*/

        let collision_box_dimensions = (0.25, 1.6);
        
        let mut potential_position = self.position + delta;

        let player_bounding_box = crate::collision::Rect3 {
            pos: Vector3::new(
                potential_position.x - (collision_box_dimensions.0/2.0),
                potential_position.y - (collision_box_dimensions.1/2.0),
                potential_position.z - (collision_box_dimensions.0/2.0)),
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

                    if crate::collision::rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        println!("Collision!");
                        potential_position = self.position;
                    }
                }
            }
        }

        self.position = potential_position;
        self.camera.translate(self.position + 0.8 * Y_VECTOR);
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