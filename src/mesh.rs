use crate::{shader::Shader, vertex::Vertex};
use std::{ffi::c_void, path::Path};
use std::{ffi::CString, ptr};
use std::mem::size_of;
use crate::offset_of;
use gl::types::*;
use image::{self, GenericImageView};

#[derive(Clone, Copy)]
pub struct Texture {
    pub id: u32,
}

pub(crate) struct Mesh {
    pub vertices: Vec<Vertex>,
    pub texture: Texture,
    pub vao: u32,
    vbo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, texture: &Texture, shader: &Shader) -> Self {
        let mut mesh = Mesh {
            vertices, texture: texture.clone(),
            vao: 0, vbo: 0
        };
        
        unsafe { 
            mesh.setup_mesh(shader);
        }
        mesh
    }

    unsafe fn setup_mesh(&mut self, shader: &Shader) {

        gl::ActiveTexture(gl::TEXTURE0);

        gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as f32);
        gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as f32);

        let sampler = CString::new("texture_map").unwrap().as_ptr();
        gl::Uniform1i(gl::GetUniformLocation(shader.id, sampler), 0);
        gl::BindTexture(gl::TEXTURE_2D, self.texture.id);

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

pub fn texture_from_file(path: &str, directory: &str) -> u32 {
    let filename = format!("{}/{}", directory, path);

    let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
    let img = img.flipv();
    let format = match img {
        image::DynamicImage::ImageLuma8(_) => gl::RED,
        image::DynamicImage::ImageLumaA8(_) => gl::RG,
        image::DynamicImage::ImageRgb8(_) => gl::RGB,
        image::DynamicImage::ImageRgba8(_) => gl::RGBA,
        _ => panic!("Unknown image format"),
    };

    let data = img.as_bytes();

    let mut textureID = 0;
    
    unsafe {
        gl::GenTextures(1, &mut textureID);

        

        gl::BindTexture(gl::TEXTURE_2D, textureID);
        gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
            0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    textureID
}