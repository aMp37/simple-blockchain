use std::fmt::Debug;

use serde::Serialize;

use self::{block::{Block}, user_data::UserData};
mod util;
mod block;
mod user_data;
pub struct BlockChain<T> 
    where T: Serialize + Clone {
    blocks: Vec<block::Block<T>>
}

pub trait BlockChainDataManipulation<T> {
    fn insert_data(&mut self, data: T);

    fn alter_block_of_id(&mut self, id: u32, new_content: T);
}

impl<T> BlockChain<T> 
    where T: Serialize + Debug + Clone {
    pub fn new() -> Self {
        Self {
            blocks: vec![]
        }
    }

    pub fn print_blockchain(&self) {
        for ele in self.blocks.iter() {
            println!("{:?}",ele);
        }
    }
}

impl<T> BlockChainDataManipulation<T> for BlockChain<T> 
    where T: Serialize + Clone {
    fn insert_data(&mut self, data: T) {
        if self.blocks.is_empty() {
            let mut genesis_block = Block::create_genesis_block(0, data);
            genesis_block = genesis_block.mine();
            self.blocks.push(genesis_block.mine());
        } else {
            let previous_block_hash = self.blocks.last().unwrap().calculate_hash();
            let mut block = Block::create_block(self.blocks.len() as u32, &previous_block_hash, data);
            block = block.mine();
            self.blocks.push(block);
        }
    }

    fn alter_block_of_id(&mut self, id: u32, new_content: T) {

        //alter block content
        let (idx, _) = self.blocks.iter().enumerate().find(|(_, block)| block.block_id == id).unwrap();
        self.blocks[idx].data = UserData::new(new_content);
        
        //Update references
        for i in idx+1..self.blocks.len() {
            self.blocks[i].previous_block_hash = self.blocks[i-1].calculate_hash();
        }
    }
}