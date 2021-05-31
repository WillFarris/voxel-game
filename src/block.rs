use crate::mesh::Mesh;

pub const BLOCKS: [Block; 13] = [
    Block {id: 0, name: "Air",  transparent: true, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: None },
    Block {id: 1, name: "Stone", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(1.0, 15.0)) },
    Block {id: 2, name: "Grass", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::TopSideBottom((0.0, 15.0), (3.0, 15.0), (2.0, 15.0))) },
    Block {id: 3, name: "Dirt", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(2.0, 15.0)) },
    Block {id: 4, name: "Cobblestone", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(0.0, 14.0)) },
    Block {id: 5, name: "Oak Plank", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(4.0, 15.0)) },
    Block {id: 6, name: "Sand", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(2.0, 14.0)) },
    Block {id: 7, name: "Rose", transparent: true, block_type: BlockType::Grass, mesh_type: MeshType::CrossedPlanes, texture_map: Some(TextureType::Single(12.0, 15.0))},
    Block {id: 8, name: "Glass", transparent: true, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(1.0, 12.0))},
    Block {id: 9, name: "Oak Log", transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::TopAndSide((5.0, 14.0), (4.0, 14.0)))},
    Block {id: 10, name: "Dandelion", transparent: true, block_type: BlockType::Grass, mesh_type: MeshType::CrossedPlanes, texture_map: Some(TextureType::Single(13.0, 15.0))},
    Block {id: 11, name: "Oak Leaves", transparent: true, block_type: BlockType::Leaves, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(4.0, 12.0))},
    Block {id: 12, name: "Short Grass", transparent: true, block_type: BlockType::Grass, mesh_type: MeshType::CrossedPlanes, texture_map: Some(TextureType::Single(7.0, 13.0))},
];

#[derive(Clone, Copy)]
pub enum BlockType {
    Block,
    Grass,
    Leaves,
    Water,
    Lava,
}

#[derive(Clone, Copy)]
pub enum MeshType {
    Block,
    CrossedPlanes,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum TextureType {
    Single(f32, f32),
    TopAndSide((f32, f32), (f32, f32)),
    TopSideBottom((f32, f32), (f32, f32), (f32, f32)),
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub struct Block {
    pub id: usize,
    pub name: &'static str,
    pub transparent: bool,
    pub block_type: BlockType,
    pub mesh_type: MeshType,
    pub texture_map: Option<TextureType>,
}

impl Block {
    pub fn _new(id: usize, name: &'static str, transparent: bool, block_type: BlockType, mesh_type: MeshType, texture_map: Option<TextureType>) -> Self {
        Self {
            id,
            name,
            transparent,
            block_type,
            mesh_type,
            texture_map
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Air",
            transparent: true,
            block_type: BlockType::Block,
            mesh_type: MeshType::Block,
            texture_map: None
        }
    }
}