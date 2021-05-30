use cgmath::Vector3;

use crate::{camera::Camera, vectormath::{Y_VECTOR, dda, len, normalize, normalize_inplace}, world::World};


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

        let forward = self.camera.forward;//normalize(Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z));
        let right = self.camera.right;

        let mut delta = delta_time * Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * forward.x * self.direction.z as f32),
            y: self.move_speed * self.direction.y as f32,
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * forward.z * self.direction.z as f32),
        };


        /*
        let mut collision_box = Vec::with_capacity(4);
        collision_box.push((0.25 * right) + (0.25 * forward) + delta);
        collision_box.push((-0.25 * right) + (0.25 * forward) + delta);
        collision_box.push((0.25 * right) - (0.25 * forward) + delta);
        collision_box.push((-0.25 * right) - (0.25 * forward) + delta);

        for vert in collision_box {
            let collision_check_feet = Vector3::new(
                (self.position.x + delta.x + vert.x).floor() as isize,
                (self.position.y + delta.y + vert.y - 0.8).floor() as isize,
                (self.position.z + delta.z + vert.z).floor() as isize,
            );

            let collision_check_head = Vector3::new(
                (self.position.x + delta.x + vert.x).floor() as isize,
                (self.position.y + delta.y + vert.y + 0.8).floor() as isize,
                (self.position.z + delta.z + vert.z).floor() as isize,
            );
            
            if world.collision_at_world_pos(collision_check_feet) ||
               world.collision_at_world_pos(collision_check_head) {
                if let Some((global_intersect_coords, global_block_index)) = dda(&world, &(self.position + delta - 0.8 * Y_VECTOR), &vert, len(&(vert))) {

                    /*if (self.position.x + delta.x).floor() as isize == global_block_index.x {
                        delta.z = 0.0;
                    }
                    if (self.position.z + delta.z).floor() as isize == global_block_index.z {
                        delta.x = 0.0;
                    }*/
                    //let dot_player_intersect = crate::vectormath::dot(self.position, Y_VECTOR);
                    let block_delta = Vector3::new(
                        (self.position.x + delta.x + vert.x).floor() - global_block_index.x as f32,
                        (self.position.y + delta.y + vert.y - (self.height/2.0)).floor() - global_block_index.y as f32,
                        (self.position.z + delta.z + vert.z).floor() - global_block_index.z as f32,
                    );

                    delta.x -= delta.x * (1.0 - block_delta.x);
                    delta.y -= delta.y * (1.0 - block_delta.y);
                    delta.z -= delta.z * (1.0 - block_delta.z);
                    
                    if global_intersect_coords.y < self.position.y {
                        self.grounded = true;
                        self.direction.y = 0.0;
                    } else {
                        self.grounded = false;
                    }
                }
                if let Some((_global_intersect_coords, global_block_index)) = dda(&world, &(self.position + delta + 0.8 * Y_VECTOR), &vert, len(&(vert))) {
                    /*if (self.position.x + delta.x).floor() as isize == global_block_index.x {
                        delta.z = 0.0;
                    }
                    if (self.position.z + delta.z).floor() as isize == global_block_index.z {
                        delta.x = 0.0;
                    }*/
                }
            } else {
                self.grounded = false;
            }
        }

        /*let grounded_check = Vector3::new(
            self.position.x.floor() as isize,
            (self.position.y-0.9).floor() as isize,
            self.position.z.floor() as isize
        );
        if world.collision_at_world_pos(grounded_check) {
            if let Some((global_intersect_coords, global_block_index)) = dda(&world, &(self.position - 0.8 * Y_VECTOR), &(-1.0 * Y_VECTOR), 0.0) {
                self.position.y = global_intersect_coords.y + 0.8;
                self.grounded = true;
            }
        } else {
            self.grounded = false;
            delta += GRAVITY;
        }*/

        if !self.grounded {
            self.direction += GRAVITY;
        }*/
        
        self.position += delta;
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