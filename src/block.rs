
#[derive(Clone, Copy)]
pub enum TextureType {
    Single(usize, usize),
    TopAndSide((usize, usize), (usize, usize))
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