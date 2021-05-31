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

        let forward = self.camera.forward;

        let mut delta = delta_time * Vector3 {
            x: (self.move_speed * self.camera.right.x * self.direction.x as f32) + (self.move_speed * forward.x * self.direction.z as f32),
            y: self.move_speed * self.direction.y as f32,
            z: (self.move_speed * self.camera.right.z * self.direction.x as f32) + (self.move_speed * forward.z * self.direction.z as f32),
        };
        let world_pos = Vector3 {
            x: (self.position.x + delta.x).floor() as isize,
            y: (self.position.y + delta.y).floor() as isize,
            z: (self.position.z + delta.z).floor() as isize,
        };
        //if world.collision_at_world_pos(world_pos) {
        if let Some((intersect_position, world_index)) = dda(&world, &self.position, &delta, len(&delta)) {
            
            let normal = if intersect_position.x == world_index.x as f32 {
                Vector3::new(-1.0f32, 0.0, 0.0)
            } else if intersect_position.x-1.0 == world_index.x as f32 {
                Vector3::new(1.0f32, 0.0, 0.0)
            } else if intersect_position.y == world_index.y as f32 {
                Vector3::new(0.0, -1.0, 0.0)
            } else if intersect_position.y-1.0 == world_index.y as f32 {
                Vector3::new(0.0, -1.0, 0.0)
            } else if intersect_position.z == world_index.z as f32 {
                Vector3::new(0.0, 0.0, -1.0)
            } else if intersect_position.z-1.0 == world_index.z as f32 {
                Vector3::new(0.0, 0.0, 1.0)
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            };

            
        }
        else {
            self.grounded = false;
        }
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