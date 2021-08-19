use core::fmt;
use std::{convert::TryInto, vec};
use serde::Serialize;

use super::util::{Sha256Digest, common_utils::get_actual_timestamp, sha_256_provider::{Sha256Hasher, Sha256Provider}};
use super::user_data::{UserData};

#[derive(Debug, Clone)]
pub struct BlockFullError;

impl fmt::Display for BlockFullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block has reached his max size")
    }
}

pub trait SizeConstrained {
    fn max_size() -> usize;
}

#[derive(Debug, Clone)]
pub(super) struct Block<T> 
    where T: Serialize + Clone, Self: SizeConstrained {

    pub block_id: u32,
    pub previous_block_hash: Sha256Digest,
    pub timestamp: u128,
    pub data: Vec<UserData<T>>,
    pub nonce: u32
}

impl<T> Block<T>
    where T: Serialize + Clone, Self: SizeConstrained {
    pub(super) fn create_genesis_block(id: u32) -> Block<T> {
        Block::new(id, None)
    }
    
    pub(super) fn create_block(id: u32, previous_block_hash: &[u8;32]) -> Block<T> {
        Block::new(id, Some(previous_block_hash))
    }

    fn new(id: u32, prev_block_hash: Option<&Sha256Digest>) -> Self {
        let prev_block_hash = match prev_block_hash {
            Some(prev_block_hash_ref) => *prev_block_hash_ref,
            _ => [0u8;32],
        };

        Self {
            block_id: id,
            previous_block_hash: prev_block_hash,
            timestamp: get_actual_timestamp(),
            data: vec![],
            nonce: 0u32
        }
    }

    pub fn push_data_to_block(&mut self, data: T) -> Result<(), BlockFullError> {
        if self.data.len() < Self::max_size() {
            self.data.push(UserData::new(data));
            Ok(())
        } else {
            Err(BlockFullError)
        }
    }

    pub(super) fn calculate_hash(&self) -> Sha256Digest {
        let mut sha256hasher = Sha256Hasher::default_hasher();
        sha256hasher.update(&self.block_id.to_be_bytes());
        sha256hasher.update(Self::serialize_block_data_to_json(&self.data));
        sha256hasher.update(self.timestamp.to_be_bytes());
        sha256hasher.update(&self.previous_block_hash);
        sha256hasher.update(self.nonce.to_be_bytes());

        sha256hasher.finalize().try_into().unwrap()
    }

    fn serialize_block_data_to_json<R>(data: &Vec<R>) -> String
        where R: Serialize {
        data.iter()
            .map(serde_json::to_string)
            .map(Result::unwrap)
            .collect()
    }

    pub(super) fn mine(&self) -> Block<T> {
        let mut mined_block = self.clone();
        let mut nonce_candidate = 0u32;
        while !mined_block.is_valid() {
            nonce_candidate += 1u32;
            mined_block.nonce = nonce_candidate;
        }
        mined_block
    }

    pub(super) fn is_valid(&self) -> bool {
        self.calculate_hash()[30..32] == [0,0]
    }
}

#[cfg(test)]
mod test {

}