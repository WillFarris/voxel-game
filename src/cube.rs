use crate::{camera::Camera, vertex::*};

const CUBE_VERTICES: [Vertex; 36] = [
    Vertex { position: (-0.5,  0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-top-left
    Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-top-right
    Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left

    Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-top-right
    Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left
    Vertex { position: ( 0.5, -0.5,  0.5), normal: ( 0.0,  0.0,  1.0), tex_coords: (0.0, 0.0) },   // Front-bottom-right
    
    Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left
    Vertex { position: ( 0.5, -0.5,  0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-bottom-right
    Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right

    Vertex { position: ( 0.5, -0.5,  0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-bottom-right
    Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right
    Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-top-right

    Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right
    Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-top-right
    Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-right

    Vertex { position: ( 0.5,  0.5,  0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-top-right
    Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-right
    Vertex { position: (-0.5,  0.5,  0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-top-left

    Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-right
    Vertex { position: (-0.5,  0.5,  0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-top-left
    Vertex { position: (-0.5,  0.5, -0.5), normal: ( 0.0,  1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-left

    Vertex { position: (-0.5,  0.5,  0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-top-left
    Vertex { position: (-0.5,  0.5, -0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-left
    Vertex { position: (-0.5, -0.5,  0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left

    Vertex { position: (-0.5,  0.5, -0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-top-left
    Vertex { position: (-0.5, -0.5,  0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left
    Vertex { position: (-0.5, -0.5, -0.5), normal: ( -1.0,  0.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-left

    Vertex { position: (-0.5, -0.5,  0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Front-bottom-left
    Vertex { position: (-0.5, -0.5, -0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-left
    Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  -1.0, 0.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right

    Vertex { position: (-0.5, -0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) },   // Back-bottom-left
    Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right
    Vertex { position: (-0.5,  0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) },   // Back-top-left

    Vertex { position: ( 0.5, -0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) },   // Back-bottom-right
    Vertex { position: (-0.5,  0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) },   // Back-top-left
    Vertex { position: ( 0.5,  0.5, -0.5), normal: ( 0.0,  0.0, -1.0), tex_coords: (0.0, 0.0) }     // Back-top-right
];

pub struct Cube {
    position: [f32; 3],
    color: [f32; 3],
    //texture: Option<glium::Texture2d>,

    pub model_matrix: [[f32; 4]; 4],
    //vertices: VertexBuffer<Vertex>,
}

impl Cube {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [position[0], position[1], position[2], 1.0],
            ],
            color,
        }
    }

    pub fn translate(&mut self, pos: &[f32; 3]) {
        self.position[0] = pos[0];
        self.position[1] = pos[1];
        self.position[2] = pos[2];

        self.model_matrix[3][0] = pos[0];
        self.model_matrix[3][1] = pos[1];
        self.model_matrix[3][2] = pos[2];
    }

    /*pub fn draw(
        &self,
        target: &mut glium::Frame,
        params: &DrawParameters,
        camera: &Camera,
        shader: &Program,
    ) {
        target
            .draw(
                &self.vertices,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &shader,
                &uniform! {
                    model_matrix: self.model_matrix,
                    view_matrix: camera.view_matrix(),
                    perspective_matrix: crate::camera::perspective_matrix(&target),
                    light: crate::SCENE_LIGHT,
                    u_color: self.color,
                },
                &params,
            )
            .unwrap();
    }*/
}
