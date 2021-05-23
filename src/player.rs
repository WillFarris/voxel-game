use cgmath::Vector3;

use crate::{camera::Camera, vectormath::{Y_VECTOR, dda, len, normalize_inplace}, world::World};


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

    pub fn update(&mut self, world: &World) {

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

        for vert in collision_box {
            let collision_check_feet = Vector3::new(
                (self.position.x + delta.x + vert.x) as isize,
                (self.position.y + delta.y + vert.y) as isize,
                (self.position.z + delta.z + vert.z) as isize,
            );

            let collision_check_head = Vector3::new(
                (self.position.x + delta.x + vert.x) as isize,
                (self.position.y + delta.y + vert.y + 1.8) as isize,
                (self.position.z + delta.z + vert.z) as isize,
            );
            
            if world.collision_at_world_pos(collision_check_feet) ||
               world.collision_at_world_pos(collision_check_head) {
                if let Some((_global_intersect_coords, global_block_index)) = dda(&world, &(self.position + delta), &vert, len(&(vert))) {

                    if (self.position.x + delta.x) as isize == global_block_index.x {
                        delta.z = 0.0;
                    }
                    if (self.position.z + delta.z) as isize == global_block_index.z {
                        delta.x = 0.0;
                    }
                }
                if let Some((_global_intersect_coords, global_block_index)) = dda(&world, &(self.position + delta + Y_VECTOR), &vert, len(&(vert))) {
                    if (self.position.x + delta.x) as isize == global_block_index.x {
                        delta.z = 0.0;
                    }
                    if (self.position.z + delta.z) as isize == global_block_index.z {
                        delta.x = 0.0;
                    }
                }
            }
        }

        let grounded_check = Vector3::new(
            self.position.x as isize,
            (self.position.y-0.1) as isize,
            self.position.z as isize
        );
        if world.collision_at_world_pos(grounded_check) {
            self.grounded = true;
        } else {
            self.grounded = false;
            delta += GRAVITY;
        }
        
        self.position += delta;
        self.camera.translate(self.position + 1.6 * Y_VECTOR);
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