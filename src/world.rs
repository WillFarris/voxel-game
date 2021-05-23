use std::collections::HashMap;

use cgmath::{Matrix4, Vector3};

use crate::{block, mesh::{self, Texture, texture_from_file}, meshgen::{self, gen_chunk_mesh}, shader::Shader};

use std::ffi::CStr;

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk<'a> {
    blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    mesh: crate::mesh::Mesh,
    model_matrix: Matrix4<f32>,
    texture: Texture,
    shader: &'a Shader,
}

impl<'a> Chunk<'a> {
    pub fn from_blocks(blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE], position: Vector3<isize>, shader: &'a Shader, texture_id: u32) -> Self {
        let texture = Texture {id: texture_id};
        let mut mesh_vertices = gen_chunk_mesh(&blocks);
        let mut mesh = mesh::Mesh::new(mesh_vertices, &texture, &shader);

        Self {
            blocks,
            mesh,
            model_matrix: Matrix4::from_translation(Vector3::new(position.x as f32, position.y as f32, position.z as f32)),
            texture,
            shader,
        }
    }

    pub unsafe fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>, shader: &Shader) {
        shader.set_mat4(c_str!("model_matrix"), &self.model_matrix);
        self.mesh.draw();
    }

    pub fn block_at_chunk_pos(&self, chunk_index: &Vector3<usize>) -> usize {
        self.blocks[chunk_index.x][chunk_index.y][chunk_index.z]
    }

    pub fn destroy_at_chunk_pos(&mut self, position: Vector3<usize>) {
        self.blocks[position.x][position.y][position.z] = 0;
        let mesh_vertices = meshgen::gen_chunk_mesh(&self.blocks);
        self.mesh = mesh::Mesh::new(mesh_vertices, &self.texture, self.shader);
    }

    pub fn place_at_chunk_pos(&mut self, position: Vector3<usize>, block_id: usize) {
        self.blocks[position.x][position.y][position.z] = block_id;
        let mesh_vertices = meshgen::gen_chunk_mesh(&self.blocks);
        self.mesh = mesh::Mesh::new(mesh_vertices, &self.texture, self.shader);
    }
}

pub struct World<'a> {
    chunks: HashMap<Vector3<isize>, Option<Chunk<'a>>>,
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

    pub fn chunk_from_block_array(&mut self, chunk_index: Vector3<isize>, blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) {
        let new_chunk = Chunk::from_blocks(blocks, 16 * chunk_index, self.shader, self.texture_id);
        self.chunks.insert(chunk_index, Some(new_chunk));
    }

    pub unsafe fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>) {
        for (position, chunk) in &self.chunks {
            if let Some(c) = chunk {
                c.render(projection_matrix, view_matrix, self.shader);
            }
        }
    }

    fn chunk_and_block_index(world_pos: &Vector3<isize>) -> (Vector3<isize>, Vector3<usize>) {
        let chunk_index = world_pos / 16;
        let block_index = Vector3 {
            x: world_pos.x as usize % CHUNK_SIZE,
            y: world_pos.y as usize % CHUNK_SIZE,
            z: world_pos.z as usize % CHUNK_SIZE,
        };
        (chunk_index, block_index)
    }

    pub fn destroy_at_global_pos(&mut self, world_pos: Vector3<isize>) {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(Some(chunk)) = self.chunks.get_mut(&chunk_index) {
            chunk.destroy_at_chunk_pos(block_index);
        }
    }

    pub fn place_at_global_pos(&mut self, world_pos: Vector3<isize>, block_id: usize) {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(Some(chunk)) = self.chunks.get_mut(&chunk_index) {
            chunk.place_at_chunk_pos(block_index, block_id);
        }
    }

    pub fn block_at_global_pos(&self, world_pos: Vector3<isize>) -> usize {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(chunk) = &self.chunks[&chunk_index] {
            return chunk.block_at_chunk_pos(&block_index);
        }
        0
    }

    pub fn collision_at_world_pos(&self, world_pos: Vector3<isize>) -> bool {
        self.block_at_global_pos(world_pos) != 0
    }
}