use crate::vertex::Vertex;
use glium::Texture2d;

struct _Cube {
    id: usize,
    position: [f32; 3],
    texture: glium::Texture2d,
}

pub const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
];

impl _Cube {
    pub fn new(id: usize, position: [f32; 3], texture: Texture2d) -> Self {
        Self {
            id,
            position,
            texture,
        }
    }
}
