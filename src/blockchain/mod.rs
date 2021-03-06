use self::block::{create_block, create_genesis_block};
type Sha256Digest = [u8;32];
mod block;
pub struct BlockChain {
    blocks: Vec<block::Block>
}

pub trait BlockChainDataManipulation {
    fn insert_data(&mut self, data: String);

    fn alter_block_of_id(&mut self, id: u32, new_content: String);
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![]
        }
    }

    pub fn print_blockchain(&self) {
        for ele in self.blocks.iter() {
            println!("{}",ele);
        }
    }
}

impl BlockChainDataManipulation for BlockChain {
    fn insert_data(&mut self, data: String) {
        if self.blocks.is_empty() {
            let mut genesis_block = create_genesis_block(0, data);
            genesis_block = genesis_block.mine();
            self.blocks.push(genesis_block.mine());
        } else {
            let previous_block_hash = self.blocks.last().unwrap().calculate_hash();
            let mut block = create_block(self.blocks.len() as u32, &previous_block_hash, data);
            block = block.mine();
            self.blocks.push(block);
        }
    }

    fn alter_block_of_id(&mut self, id: u32, new_content: String) {

        //alter block content
        let (idx, _) = self.blocks.iter().enumerate().find(|(_, block)| block.block_id == id).unwrap();
        self.blocks[idx].data = new_content;
        
        //Update references
        for i in idx+1..self.blocks.len() {
            self.blocks[i].previous_block_hash = self.blocks[i-1].calculate_hash();
        }
    }
}