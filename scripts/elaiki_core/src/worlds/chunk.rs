use std::collections::HashMap;

use elaiki_api::utils::errors::*;
use gdnative::prelude::Vector2;

use super::Block;

pub struct Chunk {
    cell_list: HashMap<(u16, u16), Block>, // (x, y)

    size: u16,
}

impl Chunk {
    pub fn new(size: u16) -> Self {
        let hash_map_size = (size as usize).pow(2);
        Chunk {
            cell_list: HashMap::with_capacity(hash_map_size),
            size,
        }
    }
}

impl Chunk {
    pub fn get_block(&self, point: Vector2) -> Option<&Block> {
        let key = (point.x as u16, point.y as u16);
        if key.0 >= self.size || key.1 >= self.size {
            return None;
        }
        let block = self.cell_list.get(&key);
        block
    }

    pub fn get_block_mut(&mut self, point: Vector2) -> Option<&mut Block> {
        let key = (point.x as u16, point.y as u16);
        if key.0 >= self.size || key.1 >= self.size {
            return None;
        }
        let block = self.cell_list.get_mut(&key);
        block
    }

    pub fn set_block(&mut self, point: Vector2, block: Block) -> Result<(), Error> {
        let key = (point.x as u16, point.y as u16);
        if key.0 >= self.size || key.1 >= self.size {
            return Err(err!("block point out of bounds."));
        }
        self.cell_list.insert(key, block);
        Ok(())
    }
}
