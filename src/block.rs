use crate::mesh::Mesh;



pub const BLOCKS: [Block; 7] = [
    Block {id: 0, name: "Air",  transparent: true, mesh_type: MeshType::Block, texture_map: None },
    Block {id: 1, name: "Stone", transparent: false, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(1,15)) },
    Block {id: 2, name: "Grass", transparent: false, mesh_type: MeshType::Block, texture_map: Some(TextureType::TopSideBottom((0,15), (3, 15), (2, 15))) },
    Block {id: 3, name: "Dirt", transparent: false, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(2,15)) },
    Block {id: 4, name: "Cobblestone", transparent: false, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(0,14)) },
    Block {id: 5, name: "Oak Plank", transparent: false, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(4,15)) },
    Block {id: 6, name: "Sand", transparent: false, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(0,1)) },
];

#[derive(Clone, Copy)]
pub enum MeshType {
    Block,
    CrossedPlanes,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum TextureType {
    Single(usize, usize),
    TopAndSide((usize, usize), (usize, usize)),
    TopSideBottom((usize, usize), (usize, usize), (usize, usize)),
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub struct Block {
    pub id: usize,
    pub name: &'static str,
    pub transparent: bool,
    pub mesh_type: MeshType,
    pub texture_map: Option<TextureType>,
}

impl Block {
    pub fn _new(id: usize, name: &'static str, transparent: bool, mesh_type: MeshType, texture_map: Option<TextureType>) -> Self {
        Self {
            id,
            name,
            transparent,
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
            mesh_type: MeshType::Block,
            texture_map: None
        }
    }
}