use crate::vertex::*;

const CUBE_FACES: [[Vertex; 6]; 6] = [
    
    // Facing positive-X
    [
        Vertex { position: ( 0.5,  0.5,  0.5) },   // Front-top-right
        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right
        Vertex { position: ( 0.5, -0.5,  0.5) },   // Front-bottom-right

        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right
        Vertex { position: ( 0.5,  0.5,  0.5) },   // Front-top-right
        Vertex { position: ( 0.5,  0.5, -0.5) },   // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex { position: (-0.5,  0.5,  0.5) },   // Front-top-left
        Vertex { position: (-0.5,  0.5, -0.5) },   // Back-top-left
        Vertex { position: (-0.5, -0.5,  0.5) },   // Front-bottom-left

        Vertex { position: (-0.5,  0.5, -0.5) },   // Back-top-left
        Vertex { position: (-0.5, -0.5,  0.5) },   // Front-bottom-left
        Vertex { position: (-0.5, -0.5, -0.5) },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex { position: ( 0.5,  0.5,  0.5) },   // Front-top-right
        Vertex { position: ( 0.5,  0.5, -0.5) },   // Back-top-right
        Vertex { position: (-0.5,  0.5,  0.5) },   // Front-top-left

        Vertex { position: ( 0.5,  0.5, -0.5) },   // Back-top-right
        Vertex { position: (-0.5,  0.5,  0.5) },   // Front-top-left
        Vertex { position: (-0.5,  0.5, -0.5) },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex { position: (-0.5, -0.5,  0.5) },   // Front-bottom-left
        Vertex { position: (-0.5, -0.5, -0.5) },   // Back-bottom-left
        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right

        Vertex { position: (-0.5, -0.5,  0.5) },   // Front-bottom-left
        Vertex { position: ( 0.5, -0.5,  0.5) },   // Front-bottom-right
        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex { position: (-0.5,  0.5,  0.5) },   // Front-top-left
        Vertex { position: ( 0.5,  0.5,  0.5) },   // Front-top-right
        Vertex { position: (-0.5, -0.5,  0.5) },   // Front-bottom-left

        Vertex { position: ( 0.5,  0.5,  0.5) },   // Front-top-right
        Vertex { position: (-0.5, -0.5,  0.5) },   // Front-bottom-left
        Vertex { position: ( 0.5, -0.5,  0.5) },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex { position: (-0.5, -0.5, -0.5) },   // Back-bottom-left
        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right
        Vertex { position: (-0.5,  0.5, -0.5) },   // Back-top-left

        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right
        Vertex { position: (-0.5,  0.5, -0.5) },   // Back-top-left
        Vertex { position: ( 0.5,  0.5, -0.5) }     // Back-top-right
    ],
];

const CUBE_NORMALS: [[Normal; 6]; 6] = [

    // Facing positive-X
    [
        Normal { normal: ( 1.0,  0.0,  0.0) },   // Back-bottom-right
        Normal { normal: ( 1.0,  0.0,  0.0) },   // Front-top-right
        Normal { normal: ( 1.0,  0.0,  0.0) },   // Back-top-right

        Normal { normal: ( 1.0,  0.0,  0.0) },   // Front-bottom-right
        Normal { normal: ( 1.0,  0.0,  0.0) },   // Back-bottom-right
        Normal { normal: ( 1.0,  0.0,  0.0) },   // Front-top-right

    ],

    // Facing negative-X
    [
        Normal { normal: (-1.0,  0.0,  0.0) },   // Front-top-left
        Normal { normal: (-1.0,  0.0,  0.0) },   // Back-top-left
        Normal { normal: (-1.0,  0.0,  0.0) },   // Front-bottom-left
    
        Normal { normal: (-1.0,  0.0,  0.0) },   // Back-top-left
        Normal { normal: (-1.0,  0.0,  0.0) },   // Front-bottom-left
        Normal { normal: (-1.0,  0.0,  0.0) },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Normal { normal: ( 0.0,  1.0,  0.0) },   // Front-top-right
        Normal { normal: ( 0.0,  1.0,  0.0) },   // Back-top-right
        Normal { normal: ( 0.0,  1.0,  0.0) },   // Front-top-left
    
        Normal { normal: ( 0.0,  1.0,  0.0) },   // Back-top-right
        Normal { normal: ( 0.0,  1.0,  0.0) },   // Front-top-left
        Normal { normal: ( 0.0,  1.0,  0.0) },   // Back-top-left
    ],

    // Facing negative-Y
    [
        Normal { normal: ( 0.0, -1.0,  0.0) },   // Front-bottom-left
        Normal { normal: ( 0.0, -1.0,  0.0) },   // Front-bottom-right
        Normal { normal: ( 0.0, -1.0,  0.0) },   // Back-bottom-right
    
        Normal { normal: ( 0.0, -1.0,  0.0) },   // Front-bottom-left
        Normal { normal: ( 0.0, -1.0,  0.0) },   // Back-bottom-left
        Normal { normal: ( 0.0, -1.0,  0.0) },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Normal { normal: ( 0.0,  0.0,  1.0) },   // Front-top-left
        Normal { normal: ( 0.0,  0.0,  1.0) },   // Front-top-right
        Normal { normal: ( 0.0,  0.0,  1.0) },   // Front-bottom-left

        Normal { normal: ( 0.0,  0.0,  1.0) },   // Front-top-right
        Normal { normal: ( 0.0,  0.0,  1.0) },   // Front-bottom-left
        Normal { normal: ( 0.0,  0.0,  1.0) },   // Front-bottom-right
    ],

    // Facing negative-Z
    [
        Normal { normal: ( 0.0,  0.0, -1.0) },   // Back-bottom-left
        Normal { normal: ( 0.0,  0.0, -1.0) },   // Back-bottom-right
        Normal { normal: ( 0.0,  0.0, -1.0) },   // Back-top-left

        Normal { normal: ( 0.0,  0.0, -1.0) },   // Back-bottom-right
        Normal { normal: ( 0.0,  0.0, -1.0) },   // Back-top-left
        Normal { normal: ( 0.0,  0.0, -1.0) }     // Back-top-right
    ],
];

fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex>, normals: &mut Vec<Normal>) {
    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v].clone();
        vertex.position.0 += position[0];
        vertex.position.1 += position[1];
        vertex.position.2 += position[2];
        let normal = CUBE_NORMALS[face][v].clone();

        vertices.push(vertex);
        normals.push(normal);
    }
}

pub fn gen_chunk_mesh(blocks: &[[[u8; 16]; 16]; 16]) -> (Vec<Vertex>, Vec<Normal>) {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                if blocks[x][y][y] == 0 {
                    continue;
                }

                let position = [x as f32, y as f32, z as f32];
                if x == 15 || blocks[x+1][y][z] == 0 {
                    push_face(&position, 0, &mut vertices, &mut normals);
                }
                if x == 0 || blocks[x-1][y][z] == 0 {
                    push_face(&position, 1, &mut vertices, &mut normals)
                }

                if y == 15 || blocks[x][y+1][z] == 0 {
                    push_face(&position, 2, &mut vertices, &mut normals);
                }
                if y == 0 || blocks[x][y-1][z] == 0 {
                    push_face(&position, 3, &mut vertices, &mut normals)
                }

                if z == 15 || blocks[x][y][z+1] == 0 {
                    push_face(&position, 4, &mut vertices, &mut normals);
                }
                if z == 0 || blocks[x][y][z-1] == 0 {
                    push_face(&position, 5, &mut vertices, &mut normals)
                }

                /*if x == 15 || blocks[x+1][y][z] == 0 {
                    for v in 0..6 {
                        let mut vertex = CUBE_FACES[0][v].clone();
                        vertex.position.0 += x as f32;
                        vertex.position.1 += y as f32;
                        vertex.position.2 += z as f32;
                        vertices.push(vertex);
                        normals.push(CUBE_NORMALS[0][v].clone());
                    }
                }
                if x == 0 || blocks[x-1][y][z] == 0 {
                    for v in 0..6 {
                        let mut vertex = CUBE_FACES[1][v].clone();
                        vertex.position.0 += x as f32;
                        vertex.position.1 += y as f32;
                        vertex.position.2 += z as f32;
                        vertices.push(vertex);
                        normals.push(CUBE_NORMALS[1][v].clone());
                    }
                }

                if y == 255 || blocks[x][y+1][z] == 0 {
                    for v in 0..6 {
                        let mut vertex = CUBE_FACES[2][v].clone();
                        vertex.position.0 += x as f32;
                        vertex.position.1 += y as f32;
                        vertex.position.2 += z as f32;
                        vertices.push(vertex);
                        normals.push(CUBE_NORMALS[2][v].clone());
                    }
                }
                if y == 0 || blocks[x][y-1][z] == 0 {
                    for v in 0..6 {
                        let mut vertex = CUBE_FACES[3][v].clone();
                        vertex.position.0 += x as f32;
                        vertex.position.1 += y as f32;
                        vertex.position.2 += z as f32;
                        vertices.push(vertex);
                        normals.push(CUBE_NORMALS[3][v].clone());
                    }
                }

                if z == 15 || blocks[x][y][z+1] == 0 {
                    for v in 0..6 {
                        let mut vertex = CUBE_FACES[4][v].clone();
                        vertex.position.0 += x as f32;
                        vertex.position.1 += y as f32;
                        vertex.position.2 += z as f32;
                        vertices.push(vertex);
                        normals.push(CUBE_NORMALS[4][v].clone());
                    }
                }
                if z == 0 || blocks[x][y][z-1] == 0 {
                    for v in 0..6 {
                        let mut vertex = CUBE_FACES[5][v].clone();
                        vertex.position.0 += x as f32;
                        vertex.position.1 += y as f32;
                        vertex.position.2 += z as f32;
                        vertices.push(vertex);
                        normals.push(CUBE_NORMALS[5][v].clone());
                    }
                }*/
            }
        }
    }
    println!("Num verts: {}", vertices.len());

    (vertices, normals)
}