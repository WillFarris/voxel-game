use std::collections::HashMap;

use cgmath::{Matrix4, Vector3, Vector2};
use gl::CULL_FACE;
use rand::random;

use crate::{block::{self, BLOCKS, MeshType}, mesh::*, meshgen::{self, *}, shader::Shader, vectormath::{dot, len, normalize}};

use std::ffi::CStr;

use noise::{Perlin, NoiseFn, Seedable};

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    solid_mesh: Option<Mesh>,
    transparent_mesh: Option<Mesh>,
    model_matrix: Matrix4<f32>,
    texture: Texture,
}

impl Chunk {
    pub fn from_blocks(blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE], position: Vector3<isize>, solid_mesh: Option<Mesh>, transparent_mesh: Option<Mesh>, texture_id: u32) -> Self {
        let texture = Texture {id: texture_id};
        Self {
            blocks,
            solid_mesh,
            transparent_mesh,
            model_matrix: Matrix4::from_translation(Vector3::new(position.x as f32, position.y as f32, position.z as f32)),
            texture,
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
    perlin: Perlin,
    texture: Texture,
    solid_shader: &'a Shader,
    transparent_shader: &'a Shader,
}

impl<'a> World<'a> {
    pub fn new(texture: Texture, solid_shader: &'a Shader, transparent_shader: &'a Shader, seed: u32) -> Self {
        let chunks = HashMap::new();
        let perlin = Perlin::new();
        let noise_offset = Vector2::new(
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
        );
        perlin.set_seed(seed);

        let mut world = Self {
            chunks,
            seed,
            noise_offset,
            perlin,
            texture,
            solid_shader,
            transparent_shader,
        };
        
        let chunk_radius: isize = 5;
        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_y in 0..chunk_radius {
                for chunk_z in -chunk_radius..chunk_radius {
                    let chunk_index = Vector3::new(chunk_x, chunk_y, chunk_z);
                    let chunk_data: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
                    let mut cur_chunk = Chunk::from_blocks(chunk_data, 16 * chunk_index, None, None, world.texture.id);
                    
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
        let noise_scale = 0.02;

        //println!("Generating terrain...");

        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = (block_x as isize + (chunk_index.x * CHUNK_SIZE as isize)) as f64;
                    let global_y = (block_y as isize + (chunk_index.y * CHUNK_SIZE as isize)) as f64;
                    let global_z = (block_z as isize + (chunk_index.z * CHUNK_SIZE as isize)) as f64;
                    let surface_y = 
                            5.0 * self.perlin.get([noise_scale * global_x + self.noise_offset.x, noise_scale * global_z + self.noise_offset.y])
                            //+ (50.0 * self.perlin.get([0.1 * noise_scale * self.noise_offset.x - 100.0, self.noise_offset.y - 44310.0]) + 3.0)
                            + 5.1;
                    if global_y < surface_y {
                        if global_y == surface_y.floor() {
                            chunk.blocks[block_x][block_y][block_z] = 2;
                            if rand::random::<usize>()%100 < 1 {
                                println!("trying to place tree at {} {} {}", block_x, block_y, block_z);
                                self.place_tree(Vector3::new(block_x, block_y, block_z), chunk);
                            } else if rand::random::<usize>()%100 < 10 {
                                chunk.blocks[block_x][block_y+1][block_z] = 12;
                            } else if rand::random::<usize>()%50 < 1 {
                                chunk.blocks[block_x][block_y+1][block_z] = 7;
                            }
                        } else if global_y < (surface_y/2.0).floor() {
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

        if block_index.x == 0 || block_index.x == CHUNK_SIZE-1 || block_index.z == 0 || block_index.z == CHUNK_SIZE-1 {
            println!("Can't place tree at {:?}", block_index);
            return;
        }
        
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
        let new_chunk = Chunk::from_blocks(blocks, 16 * chunk_index, None, None, self.texture.id);
        self.chunks.insert(chunk_index, new_chunk);
        self.gen_chunk_mesh(&chunk_index);
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

    pub fn render_solid(&self, player_position: Vector3<f32>, player_direction: Vector3<f32>) {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            for (position, chunk) in &self.chunks {
                //chunk.render(self.shader);
                if let Some(m) = &chunk.solid_mesh {
                    self.solid_shader.set_mat4(c_str!("model_matrix"), &chunk.model_matrix);
                    m.draw();
                }
            }
        }
    }

    pub fn render_transparent(&self) {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            for (position, chunk) in &self.chunks {
                if let Some(m) = &chunk.transparent_mesh {
                    self.transparent_shader.set_mat4(c_str!("model_matrix"), &chunk.model_matrix);
                    m.draw();
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
        }
        0
    }

    pub fn collision_at_world_pos(&self, world_pos: Vector3<isize>) -> bool {
        self.block_at_global_pos(world_pos) != 0
    }

    pub fn gen_chunk_mesh(&mut self, chunk_index: &Vector3<isize>) {
        let mut solid_vertices = Vec::new();
        let mut transparent_vertices = Vec::new();
    
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
                        let cur_vertices = if cur.transparent { &mut transparent_vertices} else {&mut solid_vertices};
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
            let solid_mesh = crate::mesh::Mesh::new(solid_vertices, &self.texture, &self.solid_shader);
            let transparent_mesh = crate::mesh::Mesh::new(transparent_vertices, &self.texture, &self.transparent_shader);
            chunk.solid_mesh = Some(solid_mesh);
            chunk.transparent_mesh = Some(transparent_mesh);
        }
    }
}
