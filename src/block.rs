
pub const DIAMOND: Block = Block {id: 69, texture_map: Some(TextureType::Single(2,12)) };
pub const MOSSY_COBBLESTONE: Block = Block {id: 42, texture_map: Some(TextureType::Single(4,13)) };
pub const WOOD_PLANK: Block = Block {id: 43, texture_map: Some(TextureType::Single(4,15)) };


#[derive(Clone, Copy)]
pub enum TextureType {
    Single(usize, usize),
}

#[derive(Copy, Clone)]
pub struct Block {
    pub(crate) id: usize,
    pub(crate) texture_map: Option<TextureType>,
}

impl Block {
    pub fn new(id: usize, texture_map: Option<TextureType>) -> Self {
        Self {
            id,
            texture_map
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: 0,
            texture_map: None
        }
    }
}