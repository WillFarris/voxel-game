use crate::{block::*, vertex::*};

const CUBE_FACES: [[Vertex; 6]; 6] = [
    
    // Facing positive-X
    [
        Vertex { position: ( 0.5, -0.5,  0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-bottom-right
        Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right
        Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (1.0, 1.0) },   // Front-top-right
    
        Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right
        Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (1.0, 1.0) },   // Front-top-right
        Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex { position: (-0.5,  0.5,  0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (1.0, 1.0) },   // Front-top-left
        Vertex { position: (-0.5,  0.5, -0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-top-left
        Vertex { position: (-0.5, -0.5,  0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-bottom-left
    
        Vertex { position: (-0.5,  0.5, -0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-top-left
        Vertex { position: (-0.5, -0.5,  0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-bottom-left
        Vertex { position: (-0.5, -0.5, -0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (1.0, 1.0) },   // Front-top-right
        Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-top-right
        Vertex { position: (-0.5,  0.5,  0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-top-left
    
        Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-top-right
        Vertex { position: (-0.5,  0.5,  0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-top-left
        Vertex { position: (-0.5,  0.5, -0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-bottom-left
        Vertex { position: ( 0.5, -0.5,  0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (1.0, 1.0) },   // Front-bottom-right
        Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-bottom-right

        Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 1.0) },   // Front-bottom-left
        Vertex { position: (-0.5, -0.5, -0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-left
        Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (1.0, 0.0) },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex { position: (-0.5,  0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 1.0) },   // Front-top-left
        Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (1.0, 1.0) },   // Front-top-right
        Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left
    
        Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (1.0, 1.0) },   // Front-top-right
        Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left
        Vertex { position: ( 0.5, -0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (1.0, 0.0) },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex { position: (-0.5, -0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) },   // Back-bottom-left
        Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (1.0, 0.0) },   // Back-bottom-right
        Vertex { position: (-0.5,  0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 1.0) },   // Back-top-left
    
        Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (1.0, 0.0) },   // Back-bottom-right
        Vertex { position: (-0.5,  0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 1.0) },   // Back-top-left
        Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (1.0, 1.0) }     // Back-top-right
    ],
];


fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex>, tex_coords: &(usize, usize)) {
    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v].clone();
        vertex.position.0 += position[0];
        vertex.position.1 += position[1];
        vertex.position.2 += position[2];

        vertex.tex_coords.0 = vertex.tex_coords.0 * 0.0625 + 0.0625 * tex_coords.0 as f32;
        vertex.tex_coords.1 = vertex.tex_coords.1 * 0.0625 + 0.0625 * tex_coords.1 as f32;

        vertices.push(vertex);
    }
}

pub fn gen_chunk_mesh(blocks: &[[[Block; 16]; 16]; 16]) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    //let mut normals = Vec::new();

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let cur = &blocks[x][y][z];
                if cur.id == 0 {
                    continue;
                }

                let tex_coords = if let Some(texture_type) = &cur.texture_map {
                    match texture_type {
                        crate::block::TextureType::Single(x, y) => (*x, *y),
                        _ => (0, 0),
                    }
                } else {
                    (0, 0)
                };


                let position = [x as f32, y as f32, z as f32];
                if x == 15 || blocks[x+1][y][z].id == 0 {
                    push_face(&position, 0, &mut vertices, &tex_coords);
                }
                if x == 0 || blocks[x-1][y][z].id == 0 {
                    push_face(&position, 1, &mut vertices, &tex_coords)
                }

                if y == 15 || blocks[x][y+1][z].id == 0 {
                    push_face(&position, 2, &mut vertices, &tex_coords);
                }
                if y == 0 || blocks[x][y-1][z].id == 0 {
                    push_face(&position, 3, &mut vertices, &tex_coords);
                }

                if z == 15 || blocks[x][y][z+1].id == 0 {
                    push_face(&position, 4, &mut vertices, &tex_coords);
                }
                if z == 0 || blocks[x][y][z-1].id == 0 {
                    push_face(&position, 5, &mut vertices, &tex_coords);
                }
            }
        }
    }
    println!("Num verts: {}", vertices.len());

    vertices
}