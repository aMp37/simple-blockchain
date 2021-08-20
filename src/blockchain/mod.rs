use core::{panic};
use std::{fmt::Debug};

use rsa::RsaPrivateKey;
use serde::Serialize;

use self::{block::{Block, SizeConstrained}};
pub mod util;
mod block;
pub mod user_data;

impl<T> SizeConstrained for Block<T> 
    where T: Serialize + Clone + Signable + Debug{
        fn max_size() -> usize {
        100
    }
}
pub struct BlockChain<T> 
    where T: Serialize + Signable + Clone + Debug {
    blocks: Vec<block::Block<T>>
}

pub trait Signable {
    fn sign_with_private_key(&mut self, author_private_key: &RsaPrivateKey);
    fn validate_signed_data(&self) -> bool;
    fn is_signed(&self) -> bool;
}

pub trait BlockChainDML<T> {
    fn insert_data(&mut self, data: T);

    unsafe fn alter_block_of_id(&mut self, id: u32, new_content: T);
}

impl<T> BlockChain<T> 
    where T: Serialize + Signable + Debug + Clone {
    pub fn new() -> Self {
        Self {
            blocks: vec![]
        }
    }

    pub fn validate_blockchain(&self) -> bool {
        self.blocks.iter().map(Block::<T>::is_valid)
        .fold(true, |acc, value|acc && value)
    }

    pub fn print_blockchain(&self) {
        for ele in self.blocks.iter() {
            println!("{:?}",ele);
        }
    }
}

impl<T> BlockChainDML<T> for BlockChain<T> 
    where T: Serialize + Signable + Clone + Debug{
    fn insert_data(&mut self, data: T) {
        if self.blocks.is_empty() {
            let mut genesis_block = Block::create_genesis_block(0);
            genesis_block.push_data_to_block(data).expect("Unexpected error occurred");
            self.blocks.push(genesis_block.mine());
        } else {
            if let Err(_) = self.blocks.last_mut().unwrap().push_data_to_block(data) { 
                let previous_block_hash = self.blocks.last().unwrap().calculate_hash();
                let mut block = Block::create_block(self.blocks.len() as u32, &previous_block_hash);
                block = block.mine();
                self.blocks.push(block);
             }
        }
    }

    unsafe fn alter_block_of_id(&mut self, id: u32, new_content: T) {
        //alter block content
        let (idx, _) = self.blocks.iter().enumerate().find(|(_, block)| block.block_id == id).unwrap();
        if let Err(_) =  self.blocks[idx].push_data_to_block(new_content) {
                panic!("Unsafe block alter operation unsuccessful - block is full!")
        }
        
        //Update references
        for i in idx+1..self.blocks.len() {
            self.blocks[i].previous_block_hash = self.blocks[i-1].calculate_hash();
        }
    }
}