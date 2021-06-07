pub struct Inventory {
    items: [Option<(usize, usize)>; 9],
    selected: usize,
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
}