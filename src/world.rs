use std::collections::HashMap;

use cgmath::{Matrix4, Vector3};

use crate::{mesh::{self, Texture, texture_from_file}, meshgen::{self, gen_chunk_mesh}, shader::Shader};

use std::ffi::CStr;

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    pub blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    mesh: crate::mesh::Mesh,
    model_matrix: Matrix4<f32>,
}

impl Chunk {
    pub fn from_blocks(blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE], position: Vector3<isize>, shader: &Shader, texture_id: u32) -> Self {
        let mesh_texture = Texture {id: texture_id};
        let mut mesh_vertices = gen_chunk_mesh(&blocks);
        let mut mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);

        Self {
            blocks,
            mesh,
            model_matrix: Matrix4::from_translation(Vector3::new(position.x as f32, position.y as f32, position.z as f32)),
        }
    }
    

    pub unsafe fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>, shader: &Shader) {
        shader.set_mat4(c_str!("model_matrix"), &self.model_matrix);
        self.mesh.draw();
    }
}

pub struct World<'a> {
    pub chunks: HashMap<Vector3<isize>, Option<Chunk>>,
    texture_id: u32,
    shader: &'a Shader,
}

impl<'a> World<'a> {
    pub fn new(texture_id: u32, shader: &'a Shader) -> Self {
        Self {
            chunks: HashMap::new(),
            texture_id,
            shader,
        }
    }

    pub fn chunk_from_block_array(&mut self, chunk_world_pos: Vector3<isize>, blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) {
        let new_chunk = Chunk::from_blocks(blocks, chunk_world_pos.clone(), self.shader, self.texture_id);
        self.chunks.insert(chunk_world_pos, Some(new_chunk));
    }

    pub unsafe fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>) {
        for (position, chunk) in &self.chunks {
            if let Some(c) = chunk {
                c.render(projection_matrix, view_matrix, self.shader);
            }
        }
    }

    pub fn destroy_at_global_pos(&self, world_pos: Vector3<usize>) {
        //chunk[block_index.x as usize][block_index.y as usize][block_index.z as usize] = 0;
        //mesh_vertices = meshgen::gen_chunk_mesh(&chunk);
        //mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);
        //println!("block hit: {:?}", block_index);
    }
}