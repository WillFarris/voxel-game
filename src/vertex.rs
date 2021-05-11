use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
    pub tex_coords: (f32, f32),
}

implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: (f32, f32),
    pub tex_coords: (f32, f32),
}

implement_vertex!(Vertex2D, position, tex_coords);
