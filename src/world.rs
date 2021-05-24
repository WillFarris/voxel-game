use std::collections::HashMap;

use cgmath::{Matrix4, Vector3};

use crate::{mesh::*, meshgen::{self, *}, shader::Shader};

use std::ffi::CStr;

use noise::{Perlin, NoiseFn, Seedable};

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    mesh: Option<Mesh>,
    model_matrix: Matrix4<f32>,
    texture: Texture,
}

impl Chunk {
    pub fn from_blocks(blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE], position: Vector3<isize>, mesh: Option<Mesh>, texture_id: u32) -> Self {
        let texture = Texture {id: texture_id};
        Self {
            blocks,
            mesh,
            model_matrix: Matrix4::from_translation(Vector3::new(position.x as f32, position.y as f32, position.z as f32)),
            texture,
        }
    }

    pub unsafe fn render(&self, _projection_matrix: &Matrix4<f32>, _view_matrix: &Matrix4<f32>, shader: &Shader) {
        shader.set_mat4(c_str!("model_matrix"), &self.model_matrix);
        if let Some(m) = &self.mesh {
            m.draw();
        }
    }

    pub fn block_at_chunk_pos(&self, chunk_index: &Vector3<usize>) -> usize {
        self.blocks[chunk_index.x][chunk_index.y][chunk_index.z]
    }

    /*pub fn destroy_at_chunk_pos(&mut self, position: Vector3<usize>) {
        self.blocks[position.x][position.y][position.z] = 0;
        let mesh_vertices = meshgen::gen_chunk_mesh(&self.blocks);
        self.mesh = mesh::Mesh::new(mesh_vertices, &self.texture, self.shader);
    }*/

    /*pub fn place_at_chunk_pos(&mut self, position: Vector3<usize>, block_id: usize) {
        self.blocks[position.x][position.y][position.z] = block_id;
        let mesh_vertices = meshgen::gen_chunk_mesh(&self.blocks);
        self.mesh = mesh::Mesh::new(mesh_vertices, &self.texture, self.shader);
    }*/
}

pub struct World<'a> {
    pub chunks: HashMap<Vector3<isize>, Option<Chunk>>,
    texture: Texture,
    shader: &'a Shader,
}

impl<'a> World<'a> {
    pub fn new(texture: Texture, shader: &'a Shader, seed: usize) -> Self {
        let mut chunks = HashMap::new();
        let perlin = Perlin::new();
        perlin.set_seed(rand::random());

        let noise_x_offset = rand::random::<f64>();
        let noise_z_offset = rand::random::<f64>();
        let noise_scale = 0.03;

        let num_square_chunks: isize = 10;
        for chunk_x in 0..num_square_chunks {
            for chunk_z in 0..num_square_chunks {
                let mut cur_chunk: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
                for block_x in 0..CHUNK_SIZE {
                    for block_y in 0..CHUNK_SIZE {
                        for block_z in 0..CHUNK_SIZE {
                            let global_x = noise_scale * (block_x as isize + (chunk_x * CHUNK_SIZE as isize)) as f64;
                            let global_z = noise_scale * (block_z as isize + (chunk_z * CHUNK_SIZE as isize)) as f64;
                            let max_y = {
                                let noise_val = (10.0 * (perlin.get([global_x + noise_x_offset, global_z + noise_z_offset])+1.0)/2.0 + 1.0) as usize;
                                noise_val
                            };
                            if block_y < max_y {
                                if block_y == max_y-1 {
                                    cur_chunk[block_x][block_y][block_z] = 2;
                                } else if block_y < max_y/2 {
                                    cur_chunk[block_x][block_y][block_z] = 1;
                                } else {
                                    cur_chunk[block_x][block_y][block_z] = 3;
                                }
                            }
                        }
                    }
                }
                //world.chunk_from_block_array(Vector3::new(chunk_x as isize, 0, chunk_z as isize), cur_chunk);
                let chunk_pos = Vector3::new(chunk_x, 0, chunk_z);
                chunks.insert(chunk_pos.clone(), Some(Chunk::from_blocks(cur_chunk, 16 * chunk_pos, None, texture.id)));
            }
        }

        let mut world = Self {
            chunks,
            texture,
            shader,
        };

        let mut positions = Vec::new();
        for (position, chunk_option) in &world.chunks {
            if let Some(chunk) = chunk_option {
                positions.push(position.clone());
            }
        }

        for position in positions {
            world.gen_chunk_mesh(&position);
        }

        world
    }

    pub fn chunk_from_block_array(&mut self, chunk_index: Vector3<isize>, blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) {
        let new_chunk = Chunk::from_blocks(blocks, 16 * chunk_index, None, self.texture.id);
        self.chunks.insert(chunk_index, Some(new_chunk));
        self.gen_chunk_mesh(&chunk_index);
    }

    pub unsafe fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>) {
        for (_position, chunk) in &self.chunks {
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
            //chunk.destroy_at_chunk_pos(block_index);
            chunk.blocks[block_index.x][block_index.y][block_index.z] = 0;
            self.gen_chunk_mesh(&chunk_index);
            if block_index.x == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(1, 0, 0);
                if let Some(Some(_)) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.x == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(1, 0, 0);
                if let Some(Some(_)) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.y == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 1, 0);
                if let Some(Some(_)) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.y == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 1, 0);
                if let Some(Some(_)) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.z == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 0, 1);
                if let Some(Some(_)) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.z == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 0, 1);
                if let Some(Some(_)) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }
        }
    }

    pub fn place_at_global_pos(&mut self, world_pos: Vector3<isize>, block_id: usize) {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(Some(chunk)) = self.chunks.get_mut(&chunk_index) {
            println!("TODO: Place block");
            //chunk.place_at_chunk_pos(block_index, block_id);
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

    pub fn gen_chunk_mesh(&mut self, chunk_index: &Vector3<isize>) {
        let mut chunk_vertices = Vec::new();
    
        if let Some(Some(current_chunk)) = self.chunks.get(chunk_index) {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let i = current_chunk.blocks[x][y][z];
                        if i == 0 {
                            continue;
                        }
                        let cur = &crate::block::BLOCKS[i];
                        let tex_coords:[(usize, usize);  6] = if let Some(texture_type) = &cur.texture_map {
                            let mut coords = [(0, 0); 6];
                            match texture_type {
                                crate::block::TextureType::Single(x, y) => {
                                    for i in 0..6 {
                                        coords[i] = (*x, *y)
                                    }
                                },
                                crate::block::TextureType::TopAndSide((x_top, y_top), (x_side, y_side)) => {
                                    coords[0] = (*x_side, *y_side);
                                    coords[1] = (*x_side, *y_side);
                                    coords[2] = (*x_top, *y_top);
                                    coords[3] = (*x_side, *y_side);
                                    coords[4] = (*x_side, *y_side);
                                    coords[5] = (*x_side, *y_side);
                                },
                                crate::block::TextureType::TopSideBottom((x_top, y_top), (x_side, y_side), (x_bottom, y_bottom)) => {
                                    coords[0] = (*x_side, *y_side);
                                    coords[1] = (*x_side, *y_side);
                                    coords[2] = (*x_top, *y_top);
                                    coords[3] = (*x_bottom, *y_bottom);
                                    coords[4] = (*x_side, *y_side);
                                    coords[5] = (*x_side, *y_side);
                                },
                            }
                            coords
                        } else {
                            [(0, 0); 6]
                        };

                        let position = [x as f32, y as f32, z as f32];
                        if x < 15 {
                            if current_chunk.block_at_chunk_pos(&Vector3::new(x+1, y, z)) == 0 {
                                push_face(&position, 0, &mut chunk_vertices, &tex_coords[0]);
                            }
                        } else {
                            if let Some(Some(adjacent_chunk)) = self.chunks.get(&(*chunk_index + Vector3::new(1isize, 0, 0))) {
                                if adjacent_chunk.block_at_chunk_pos(&Vector3::new(0, y, z)) == 0 {
                                    push_face(&position, 0, &mut chunk_vertices, &tex_coords[0]);
                                }
                            }
                        }

                        if x > 0 {
                            if current_chunk.block_at_chunk_pos(&Vector3::new(x-1, y, z)) == 0 {
                                push_face(&position, 1, &mut chunk_vertices, &tex_coords[1]);
                            }
                        } else {
                            if let Some(Some(adjacent_chunk)) = self.chunks.get(&(*chunk_index - Vector3::new(1isize, 0, 0))) {
                                if adjacent_chunk.block_at_chunk_pos(&Vector3::new(CHUNK_SIZE-1, y, z)) == 0 {
                                    push_face(&position, 1, &mut chunk_vertices, &tex_coords[1]);
                                }
                            }
                        }

                        if y < 15 {
                            if current_chunk.block_at_chunk_pos(&Vector3::new(x, y+1, z)) == 0 {
                                push_face(&position, 2, &mut chunk_vertices, &tex_coords[2]);
                            }
                        } else {
                            if let Some(Some(adjacent_chunk)) = self.chunks.get(&(*chunk_index + Vector3::new(0, 1isize, 0))) {
                                if adjacent_chunk.block_at_chunk_pos(&Vector3::new(x, 0, z)) == 0 {
                                    push_face(&position, 2, &mut chunk_vertices, &tex_coords[2]);
                                }
                            }
                        }

                        if y > 0 {
                            if current_chunk.block_at_chunk_pos(&Vector3::new(x, y-1, z)) == 0 {
                                push_face(&position, 3, &mut chunk_vertices, &tex_coords[3]);
                            }
                        } else {
                            if let Some(Some(adjacent_chunk)) = self.chunks.get(&(*chunk_index - Vector3::new(0, 1isize, 0))) {
                                if adjacent_chunk.block_at_chunk_pos(&Vector3::new(x, CHUNK_SIZE-1, z)) == 0 {
                                    push_face(&position, 3, &mut chunk_vertices, &tex_coords[3]);
                                }
                            }
                        }

                        if z < 15 {
                            if current_chunk.block_at_chunk_pos(&Vector3::new(x, y, z+1)) == 0 {
                                push_face(&position, 4, &mut chunk_vertices, &tex_coords[4]);
                            }
                        } else {
                            if let Some(Some(adjacent_chunk)) = self.chunks.get(&(*chunk_index + Vector3::new(0, 0, 1isize))) {
                                if adjacent_chunk.block_at_chunk_pos(&Vector3::new(x, y, 0)) == 0 {
                                    push_face(&position, 4, &mut chunk_vertices, &tex_coords[4]);
                                }
                            }
                        }

                        if z > 0 {
                            if current_chunk.block_at_chunk_pos(&Vector3::new(x, y, z-1)) == 0 {
                                push_face(&position, 5, &mut chunk_vertices, &tex_coords[5]);
                            }
                        } else {
                            if let Some(Some(adjacent_chunk)) = self.chunks.get(&(*chunk_index - Vector3::new(0, 0, 1isize))) {
                                if adjacent_chunk.block_at_chunk_pos(&Vector3::new(x, y, CHUNK_SIZE-1)) == 0 {
                                    push_face(&position, 5, &mut chunk_vertices, &tex_coords[5]);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            return;
        }

        if let Some(Some(chunk)) = self.chunks.get_mut(chunk_index) {
            let chunk_mesh = crate::mesh::Mesh::new(chunk_vertices, &self.texture, &self.shader);
            chunk.mesh = Some(chunk_mesh);
        }
    }
}
