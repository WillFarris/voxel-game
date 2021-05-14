

pub const BLOCKS: [Block; 7] = [
    Block {id: 0, name: "Air", texture_map: None },
    Block {id: 1, name: "Stone", texture_map: Some(TextureType::Single(1,15)) },
    Block {id: 2, name: "Grass", texture_map: Some(TextureType::TopSideBottom((0,15), (3, 15), (2, 15))) },
    Block {id: 3, name: "Dirt", texture_map: Some(TextureType::Single(2,15)) },
    Block {id: 4, name: "Cobblestone", texture_map: Some(TextureType::Single(0,14)) },
    Block {id: 5, name: "Oak Plank", texture_map: Some(TextureType::Single(4,15)) },
    Block {id: 6, name: "Sand", texture_map: Some(TextureType::Single(0,1)) },

];

#[derive(Clone, Copy)]
pub enum TextureType {
    Single(usize, usize),
    TopAndSide((usize, usize), (usize, usize)),
    TopSideBottom((usize, usize), (usize, usize), (usize, usize)),
}

#[derive(Copy, Clone)]
pub struct Block {
    pub(crate) id: usize,
    pub(crate) name: &'static str,
    pub(crate) texture_map: Option<TextureType>,
}

impl Block {
    pub fn _new(id: usize, name: &'static str, texture_map: Option<TextureType>) -> Self {
        Self {
            id,
            name,
            texture_map
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Air",
            texture_map: None
        }
    }
}