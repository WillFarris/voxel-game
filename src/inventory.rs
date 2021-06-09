use crate::block;

pub struct Inventory {
    pub items: [Option<(usize, usize)>; 9],
    pub selected: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self{
            items: [None; 9],
            selected: 0,
        }
    }

    pub fn add_to_inventory(&mut self, id_to_add: usize) {
        let mut first_free_index  = 0;
        for i in 0..self.items.len() {
            if let Some((id, quantiy)) = &mut self.items[i] {
                if *id == id_to_add {
                    *quantiy += 1;
                    return;
                } else {
                    first_free_index += 1;
                }
            }
        }
        if first_free_index < self.items.len() {
            self.items[first_free_index] = Some((id_to_add, 1));
        }
    }

    //TODO: This deletes 1 more block than it should
    pub fn consume_currently_selected(&mut self) -> Option<usize> {
        let mut should_delete = false;
        let mut block_id = 0;
        if let Some((id, quantity)) = &mut self.items[self.selected] {
            *quantity -= 1;
            if *quantity == 0 {
                should_delete = true;
                block_id = *id;
            } else {
                return Some(*id)
            }
        }
        if should_delete {
            self.items[self.selected] = None;
            return Some(block_id);
        }
        None
    }

    /*pub fn render_inventory(&self) {

    }*/
}