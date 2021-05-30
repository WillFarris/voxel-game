use crate::{block::*, vertex::*, world::{CHUNK_SIZE, Chunk}};
use cgmath::{Vector2, Vector3};

const CUBE_FACES: [[Vertex; 6]; 10] = [
    
    // Facing positive-X
    [
        Vertex { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },  // Front-bottom-right
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) }, // Front-top-right
    
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) }, // Front-top-right
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },  // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) }, // Front-top-left
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },  // Back-top-left
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },  // Front-bottom-left
        
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },  // Front-bottom-left
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },  // Back-top-left
        Vertex { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-top-right
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
    
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-top-right
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-bottom-right
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right

        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-bottom-left
        Vertex { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Front-top-left
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Front-bottom-left
    
        Vertex { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) },   // Front-top-right
        Vertex { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Front-bottom-left
        Vertex { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) },   // Back-bottom-left
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left
    
        Vertex { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) },   // Back-bottom-right
        Vertex { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) },   // Back-top-left
        Vertex { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0) }     // Back-top-right
    ],

    // Diagonal (0, 0) -> (1, 1)
    [
        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) },
        Vertex { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 0.0) },

        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 1.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) },
    ],

    // Diagonal (1, 1) -> (0, 0)
    [
        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) },

        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) },
        Vertex { position: Vector3::new(0.853553390593, 1.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) },
    ],

    // Diagonal (0, 1) -> (1, 0)
    [
        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) },
        Vertex { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) },

        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 1.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) },
    ],

    // Diagonal (1, 0) -> (0, 1)
    [
        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },

        Vertex { position: Vector3::new(0.146446609407, 1.0, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },
        Vertex { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) },
        Vertex { position: Vector3::new(0.853553390593, 1.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) },
    ],
];

pub fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex>, texmap_offset: &(f32, f32)) {

    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v];
        vertex.position.x += position[0];
        vertex.position.y += position[1];
        vertex.position.z += position[2];

        vertex.tex_coords.x = vertex.tex_coords.x * 0.0625 + 0.0625 * texmap_offset.0 as f32;
        vertex.tex_coords.y = vertex.tex_coords.y * 0.0625 + 0.0625 * texmap_offset.1 as f32;

        vertices.push(vertex);
    }
}