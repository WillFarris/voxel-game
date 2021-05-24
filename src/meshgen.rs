use crate::{block::*, vertex::*, world::CHUNK_SIZE};
use cgmath::{Vector2, Vector3};

const CUBE_FACES: [[Vertex; 6]; 6] = [
    
    // Facing positive-X
    [
        Vertex { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Front-bottom-right
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
    
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Back-top-left
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Front-bottom-left
    
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Back-top-left
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Front-bottom-left
        Vertex { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-top-right
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
    
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-top-right
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-bottom-right
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right

        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-bottom-left
        Vertex { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Front-bottom-left
    
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Front-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left
    
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0) }     // Back-top-right
    ],
];

pub fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex>, texmap_offset: &(usize, usize)) {
    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v].clone();
        vertex.position.x += position[0];
        vertex.position.y += position[1];
        vertex.position.z += position[2];

        vertex.tex_coords.x = vertex.tex_coords.x * 0.0625 + 0.0625 * texmap_offset.0 as f32;
        vertex.tex_coords.y = vertex.tex_coords.y * 0.0625 + 0.0625 * texmap_offset.1 as f32;

        vertices.push(vertex);
    }
}