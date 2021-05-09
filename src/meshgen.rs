

const CUBE_FACES: [[Vertex; 6]; 6] = [
    
    // Facing positive-X
    [
        Vertex { position: ( 0.5, -0.5,  0.5) },   // Front-bottom-right
        Vertex { position: ( 0.5, -0.5, -0.5) },   // Back-bottom-right
        Vertex { position: ( 0.5,  0.5,  0.5) },   // Front-top-right

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

pub fn gen_chunk_mesh(blocks: &[[u8; 16]; 16]) -> (Vec<Vertex>, Vec<Normal>) {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();

    for x in 0..16 {
        for z in 0..16 {
            if blocks[x][z] == 0 {
                continue;
            }
            
            for face in &CUBE_FACES {
                for vert in face {
                    vertices.push(vert.copy());
                }
            }
            
            for face in &CUBE_NORMALS {
                for norm in face {
                    normals.push(norm.copy());
                }
            }
        }
    }

    (vertices, normals)
}