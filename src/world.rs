use std::collections::HashMap;

use cgmath::{Matrix4, Vector3, Vector2};
use crate::{block::{self, BLOCKS, MeshType}, mesh::*, meshgen::*, shader::Shader, vectormath::{dot, len, normalize}};

use std::ffi::CStr;

use noise::{Perlin, NoiseFn, Seedable};

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    block_mesh: Option<Mesh>,
    grass_mesh: Option<Mesh>,
    leaves_mesh: Option<Mesh>,
    model_matrix: Matrix4<f32>,
}

impl Chunk {
    pub fn from_blocks(blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE], position: Vector3<isize>) -> Self {
        Self {
            blocks,
            block_mesh: None,
            grass_mesh: None,
            leaves_mesh: None,
            model_matrix: Matrix4::from_translation(Vector3::new(position.x as f32, position.y as f32, position.z as f32)),
        }
    }

    pub fn block_at_chunk_pos(&self, chunk_index: &Vector3<usize>) -> usize {
        self.blocks[chunk_index.x][chunk_index.y][chunk_index.z]
    }
}

pub struct World<'a> {
    pub chunks: HashMap<Vector3<isize>, Chunk>,
    seed: u32,
    noise_offset: Vector2<f64>,
    noise_scale: f64,
    perlin: Perlin,
    texture: Texture,
    block_shader: &'a Shader,
    grass_shader: &'a Shader,
    leaves_shader: &'a Shader,
}

impl<'a> World<'a> {
    pub fn new(texture: Texture, block_shader: &'a Shader, grass_shader: &'a Shader, leaves_shader: &'a Shader, seed: u32) -> Self {
        let chunks = HashMap::new();
        let perlin = Perlin::new();
        let noise_scale = 0.02;
        let noise_offset = Vector2::new(
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
        );
        perlin.set_seed(seed);

        let mut world = Self {
            chunks,
            seed,
            noise_offset,
            noise_scale,
            perlin,
            texture,
            block_shader,
            grass_shader,
            leaves_shader,
        };
        
        let chunk_radius: isize = 5;
        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_y in 0..chunk_radius {
                for chunk_z in -chunk_radius..chunk_radius {
                    let chunk_index = Vector3::new(chunk_x, chunk_y, chunk_z);
                    let chunk_data: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
                    let mut cur_chunk = Chunk::from_blocks(chunk_data, 16 * chunk_index);
                    
                    world.gen_terrain(&chunk_index, &mut cur_chunk);
                    //world.gen_caves(&chunk_index, &mut cur_chunk);
                    world.chunks.insert(chunk_index, cur_chunk);
                }
            }
        }

        let mut positions = Vec::new();
        for (position, chunk_option) in &world.chunks {
            positions.push(position.clone());
        }

        for position in positions {
            world.gen_chunk_mesh(&position);
        }

        world
    }

    fn gen_terrain(&mut self, chunk_index: &Vector3<isize>, chunk: &mut Chunk) {
        //let noise_scale = 0.02;

        //println!("Generating terrain...");

        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = (block_x as isize + (chunk_index.x * CHUNK_SIZE as isize)) as f64;
                    let global_y = (block_y as isize + (chunk_index.y * CHUNK_SIZE as isize)) as f64;
                    let global_z = (block_z as isize + (chunk_index.z * CHUNK_SIZE as isize)) as f64;
                    let surface_y = self.surface_noise(global_x, global_z);
                    if global_y < surface_y {
                        if global_y == surface_y.floor() {
                            chunk.blocks[block_x][block_y][block_z] = 2;
                        } else if global_y < (7.0 * surface_y/8.0).floor() {
                            chunk.blocks[block_x][block_y][block_z] = 1;
                        } else {
                            chunk.blocks[block_x][block_y][block_z] = 3;
                        }
                    }
                }
            }
        }
    }

    fn gen_caves(&mut self, chunk_index: &Vector3<isize>, chunk: &mut Chunk) {
        let noise_scale = 0.1;
        let cutoff = 0.6;

        //println!("Digging caves...");

        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = (block_x as isize + (chunk_index.x * CHUNK_SIZE as isize)) as f64;
                    let global_y = (block_y as isize + (chunk_index.y * CHUNK_SIZE as isize)) as f64;
                    let global_z = (block_z as isize + (chunk_index.z * CHUNK_SIZE as isize)) as f64;
                    let noise = self.perlin.get([noise_scale * global_x, noise_scale * global_y, noise_scale * global_z]);
                    if noise > cutoff {
                        chunk.blocks[block_x][block_y][block_z] = 0;
                    }
                }
            }
        }
    }


    pub fn place_tree(&mut self, block_index: Vector3<usize>, chunk: &mut Chunk) {

        if block_index.x == 0 || block_index.x == CHUNK_SIZE-1 || block_index.z == 0 || block_index.z == CHUNK_SIZE-1 || block_index.y > 4 {
            return;
        }
        
        chunk.blocks[block_index.x][block_index.y][block_index.z] = 3;
        for x in block_index.x-1..=block_index.x+1 {
            for z in block_index.z-1..=block_index.z+1 {
                for y in block_index.y+3..block_index.y+6 {
                    chunk.blocks[x][y][z] = 11;
                }
            }
        }
        chunk.blocks[block_index.x-1][block_index.y+5][block_index.z-1] = 0;
        chunk.blocks[block_index.x+1][block_index.y+5][block_index.z+1] = 0;
        chunk.blocks[block_index.x+1][block_index.y+5][block_index.z-1] = 0;
        chunk.blocks[block_index.x-1][block_index.y+5][block_index.z+1] = 0;
        for y in 1..5 {
            chunk.blocks[block_index.x][block_index.y+y][block_index.z] = 9;
        }
    }

    pub fn chunk_from_block_array(&mut self, chunk_index: Vector3<isize>, blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) {
        let new_chunk = Chunk::from_blocks(blocks, 16 * chunk_index);
        self.chunks.insert(chunk_index, new_chunk);
        self.gen_chunk_mesh(&chunk_index);
    }

    fn surface_noise(&self, global_x: f64, global_z: f64) -> f64 {
        5.0 * self.perlin.get([self.noise_scale * global_x + self.noise_offset.x, self.noise_scale * global_z + self.noise_offset.y])
                            //+ (50.0 * self.perlin.get([0.1 * noise_scale * self.noise_offset.x - 100.0, self.noise_offset.y - 44310.0]) + 3.0)
                            + 55.1
    }

    /*pub fn update_chunks(&mut self, player_position_global: Vector3<f32>) {
        let render_distance = 2;
        let player_block_index = Vector3::new(
            player_position_global.x.floor() as isize / 16,
            player_position_global.y.floor() as isize / 16,
            player_position_global.z.floor() as isize / 16,
        );

        for x in (player_block_index.x - render_distance)..=(player_block_index.x+render_distance) {
            for y in (player_block_index.x - render_distance)..=(player_block_index.x+render_distance) {
                for z in (player_block_index.x - render_distance)..=(player_block_index.x+render_distance) {
                    if let Some(_) = self.chunks.get(&Vector3::new(x, y, z)) {
                        continue;
                    } else {
                        self.gen_chunk(x, y, z);
                        self.gen_chunk_mesh(&Vector3::new(x, y, z));
                    }
                }
            }
        }

        self.chunks.retain(|index, chunk| {
               index.x >= (player_block_index.x - render_distance) &&
               index.x <= (player_block_index.x + render_distance) &&
               index.y >= (player_block_index.y - render_distance) &&
               index.y <= (player_block_index.y + render_distance) && 
               index.z >= (player_block_index.z - render_distance) &&
               index.z <= (player_block_index.z + render_distance)
        });

        
    }*/

    pub fn render_solid(&self, _player_position: Vector3<f32>, _player_direction: Vector3<f32>) {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            for (_position, chunk) in &self.chunks {
                //chunk.render(self.shader);
                if let Some(m) = &chunk.block_mesh {
                    self.block_shader.set_mat4(c_str!("model_matrix"), &chunk.model_matrix);
                    m.draw(&self.block_shader);
                }
            }
        }
    }

    pub fn render_grass(&self) {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            for (_position, chunk) in &self.chunks {
                if let Some(m) = &chunk.grass_mesh {
                    self.grass_shader.set_mat4(c_str!("model_matrix"), &chunk.model_matrix);
                    m.draw(&self.grass_shader);
                }
            }
        }
    }

    pub fn render_leaves(&self) {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            for (_position, chunk) in &self.chunks {
                if let Some(m) = &chunk.leaves_mesh {
                    self.leaves_shader.set_mat4(c_str!("model_matrix"), &chunk.model_matrix);
                    m.draw(&self.leaves_shader);
                }
            }
        }
    }

    /*pub unsafe fn render(&self, shader: &Shader) {
        shader.set_mat4(c_str!("model_matrix"), &self.model_matrix);
        if let Some(m) = &self.solid_mesh {
            //gl::Enable(gl::CULL_FACE);
            m.draw();
        }
        if let Some(m) = &self.transparent_mesh {
            //gl::Disable(gl::CULL_FACE);
            m.draw();
        }
    } */

    fn chunk_and_block_index(world_pos: &Vector3<isize>) -> (Vector3<isize>, Vector3<usize>) {
        let chunk_index = Vector3 {
            x: (world_pos.x as f32 / CHUNK_SIZE as f32).floor() as isize,
            y: (world_pos.y as f32 / CHUNK_SIZE as f32).floor() as isize,
            z: (world_pos.z as f32 / CHUNK_SIZE as f32).floor() as isize,
        };
        let block_index = Vector3 {
            x: (world_pos.x.rem_euclid(CHUNK_SIZE as isize)) as usize,
            y: (world_pos.y.rem_euclid(CHUNK_SIZE as isize)) as usize,
            z: (world_pos.z.rem_euclid(CHUNK_SIZE as isize)) as usize,
        };
        (chunk_index, block_index)
    }

    pub fn destroy_at_global_pos(&mut self, world_pos: Vector3<isize>) {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
            //chunk.destroy_at_chunk_pos(block_index);
            chunk.blocks[block_index.x][block_index.y][block_index.z] = 0;
            self.gen_chunk_mesh(&chunk_index);
            if block_index.x == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.x == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.y == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.y == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.z == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.z == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }
        }
    }

    pub fn place_at_global_pos(&mut self, world_pos: Vector3<isize>, block_id: usize) {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
            //chunk.destroy_at_chunk_pos(block_index);
            chunk.blocks[block_index.x][block_index.y][block_index.z] = block_id;
            self.gen_chunk_mesh(&chunk_index);
            if block_index.x == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.x == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.y == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.y == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.z == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.z == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.gen_chunk_mesh(&adjacent_chunk_index);
                }
            }
        }
    }

    pub fn block_at_global_pos(&self, world_pos: Vector3<isize>) -> usize {
        let (chunk_index, block_index) = World::chunk_and_block_index(&world_pos);
        if let Some(chunk) = self.chunks.get(&chunk_index) {
            return chunk.block_at_chunk_pos(&block_index);
        } else {
            0
        }
    }

    pub fn collision_at_world_pos(&self, world_pos: Vector3<isize>) -> bool {
        self.block_at_global_pos(world_pos) != 0
    }

    pub fn gen_chunk_mesh(&mut self, chunk_index: &Vector3<isize>) {
        let mut block_vertices = Vec::new();
        let mut grass_vertices = Vec::new();
        let mut leaves_vertices = Vec::new();

        if let Some(current_chunk) = self.chunks.get(chunk_index) {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let i = current_chunk.blocks[x][y][z];
                        if i == 0 {
                            continue;
                        }
                        let cur = &crate::block::BLOCKS[i];
                        let tex_coords:[(f32, f32);  6] = if let Some(texture_type) = &cur.texture_map {
                            let mut coords = [(0.0f32, 0.0f32); 6];
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
                            [(0.0, 0.0); 6]
                        };

                        let position = [x as f32, y as f32, z as f32];
                        let cur_vertices = match cur.block_type {
                            block::BlockType::Block => {
                                &mut block_vertices
                            },
                            block::BlockType::Grass => {
                                &mut grass_vertices
                            },
                            block::BlockType::Leaves => {
                                &mut leaves_vertices
                            }
                            _ => &mut block_vertices,
                        };
                        match cur.mesh_type {
                            MeshType::Block => {
                                
                                let x_right_adjacent = if x < 15 {
                                    Some(BLOCKS[current_chunk.block_at_chunk_pos(&Vector3::new(x+1, y, z))])
                                } else if let Some(chunk) = self.chunks.get(&(*chunk_index + Vector3::new(1isize, 0, 0))) {
                                    Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(0, y, z))])
                                } else {
                                    None
                                };
                                if let Some(adjacent_block) = x_right_adjacent {
                                    if adjacent_block.transparent && adjacent_block.id != cur.id {
                                        push_face(&position, 0, cur_vertices, &tex_coords[0]);
                                    }
                                }

                                let x_left_adjacent = if x > 0 {
                                    Some(BLOCKS[current_chunk.block_at_chunk_pos(&Vector3::new(x-1, y, z))])
                                } else if let Some(chunk) = self.chunks.get(&(*chunk_index + Vector3::new(-1isize, 0, 0))) {
                                    Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(CHUNK_SIZE-1, y, z))])
                                } else {
                                    None
                                };
                                if let Some(adjacent_block) = x_left_adjacent {
                                    if adjacent_block.transparent {
                                        push_face(&position, 1, cur_vertices, &tex_coords[1]);
                                    }
                                }

        
                                let y_top_adjacent = if y < 15 {
                                    Some(BLOCKS[current_chunk.block_at_chunk_pos(&Vector3::new(x, y+1, z))])
                                } else if let Some(chunk) = self.chunks.get(&(*chunk_index + Vector3::new(0, 1isize, 0))) {
                                    Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x,0, z))])
                                } else {
                                    None
                                };
                                if let Some(adjacent_block) = y_top_adjacent {
                                    if adjacent_block.transparent && adjacent_block.id != cur.id {
                                        push_face(&position, 2, cur_vertices, &tex_coords[2]);
                                    }
                                }
        
                                let y_bottom_adjacent = if y > 0 {
                                    Some(BLOCKS[current_chunk.block_at_chunk_pos(&Vector3::new(x, y-1, z))])
                                } else if let Some(chunk) = self.chunks.get(&(*chunk_index + Vector3::new(0, -1isize, 0))) {
                                    Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x,CHUNK_SIZE-1, z))])
                                } else {
                                    None
                                };
                                if let Some(adjacent_block) = y_bottom_adjacent {
                                    if adjacent_block.transparent {
                                        push_face(&position, 3, cur_vertices, &tex_coords[3]);
                                    }
                                }

                                let z_back_adjacent = if z < 15 {
                                    Some(BLOCKS[current_chunk.block_at_chunk_pos(&Vector3::new(x, y, z+1))])
                                } else if let Some(chunk) = self.chunks.get(&(*chunk_index + Vector3::new(0, 0, 1isize))) {
                                    Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y, 0))])
                                } else {
                                    None
                                };
                                if let Some(adjacent_block) = z_back_adjacent {
                                    if adjacent_block.transparent && adjacent_block.id != cur.id {
                                        push_face(&position, 4, cur_vertices, &tex_coords[4]);
                                    }
                                }


                                let z_front_adjacent = if z > 0 {
                                    Some(BLOCKS[current_chunk.block_at_chunk_pos(&Vector3::new(x, y, z-1))])
                                } else if let Some(chunk) = self.chunks.get(&(*chunk_index + Vector3::new(0, 0, -1isize))) {
                                    Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y, CHUNK_SIZE-1))])
                                } else {
                                    None
                                };
                                if let Some(adjacent_block) = z_front_adjacent {
                                    if adjacent_block.transparent {
                                        push_face(&position, 5, cur_vertices, &tex_coords[5]);
                                    }
                                }
                            }
                            MeshType::CrossedPlanes => {
                                push_face(&position, 6, cur_vertices, &tex_coords[0]);
                                //push_face(&position, 7, &mut transparent_vertices, &tex_coords[0]);
                                push_face(&position, 8, cur_vertices, &tex_coords[0]);
                                //push_face(&position, 9, &mut transparent_vertices, &tex_coords[0]);
                            }
                        }
                        
                    }
                }
            }
        } else {
            return;
        }

        /*transparent_vertices.sort_by(|a, b| {
            len(&a.position).partial_cmp(&len(&b.position)).unwrap()
        });*/

        if let Some(chunk) = self.chunks.get_mut(chunk_index) {
            let solid_mesh = crate::mesh::Mesh::new(block_vertices, &self.texture, &self.block_shader);
            let grass_mesh = crate::mesh::Mesh::new(grass_vertices, &self.texture, &self.grass_shader);
            let leaves_mesh = crate::mesh::Mesh::new(leaves_vertices, &self.texture, &self.leaves_shader);
            chunk.block_mesh = Some(solid_mesh);
            chunk.grass_mesh = Some(grass_mesh);
            chunk.leaves_mesh = Some(leaves_mesh);
        }
    }
}
