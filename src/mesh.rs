use crate::{shader::Shader, vertex::Vertex};
use std::ffi::c_void;
use std::{ffi::CString, mem, ptr};
use std::mem::size_of;
use crate::offset_of;
use gl::types::*;

pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

pub(crate) struct Mesh {
    pub vertices: Vec<Vertex>,
    pub textures: Vec<Texture>,
    pub vao: u32,
    vbo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, textures: Vec<Texture>) -> Self {
        let mut mesh = Mesh {
            vertices, textures,
            vao: 0, vbo: 0
        };

        unsafe { mesh.setup_mesh() }
        mesh
    }

    unsafe fn setup_mesh(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);

        gl::BindVertexArray(self.vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        let size = (self.vertices.len() * size_of::<Vertex>()) as GLsizeiptr;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        //TODO: do the same as above if indices are desired later

        let size = size_of::<Vertex>() as i32;
        // vertex Positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);
        // vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, normal) as *const c_void);
        // vertex texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tex_coords) as *const c_void);
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        gl::BindVertexArray(0);
    }
}