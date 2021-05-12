use cgmath::{Vector2, Vector3};
use cgmath::prelude::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
        }
    }
}

//implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}

//implement_vertex!(Normal, normal);
