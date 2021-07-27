use std::{convert::TryInto, fmt};
use self::block_util::get_actual_timestamp;

use super::util::{Sha256Digest, sha_256_provider::{Sha256Hasher, Sha256Provider}};

mod block_util;

#[derive(Clone)]
pub(super) struct Block {
    pub block_id: u32,
    pub previous_block_hash: Sha256Digest,
    pub timestamp: u128,
    pub data: String,
    pub nonce: u32
}

pub(super) fn create_genesis_block(id: u32, data: String) -> Block {
    Block::new(id, None, &data)
}

pub(super) fn create_block(id: u32, previous_block_hash: &[u8;32], data: String) -> Block {
    Block::new(id, Some(previous_block_hash), &data)
}

impl Block {
    fn new(id: u32, prev_block_hash: Option<&Sha256Digest>, data: &String) -> Self {
        let prev_block_hash = match prev_block_hash {
            Some(prev_block_hash_ref) => *prev_block_hash_ref,
            _ => [0u8;32],
        };

        Self {
            block_id: id,
            previous_block_hash: prev_block_hash,
            timestamp: get_actual_timestamp(),
            data: data.clone(),
            nonce: 0u32
        }
    }

    pub(super) fn calculate_hash(&self) -> Sha256Digest {
        let mut sha256hasher = Sha256Hasher::default_hasher();
        sha256hasher.update(&self.block_id.to_be_bytes());
        sha256hasher.update(&self.data);
        sha256hasher.update(self.timestamp.to_be_bytes());
        sha256hasher.update(&self.previous_block_hash);
        sha256hasher.update(self.nonce.to_be_bytes());

        sha256hasher.finalize().try_into().unwrap()
    }

    pub(super) fn mine(&self) -> Block {
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

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block #{}:
        Hash {:?}
        Previous block Hash {:?}
        Data: {}
        Nonce: {}
        Timestamp: {}
        Is Valid: {}", self.block_id ,self.calculate_hash(), self.previous_block_hash, self.data, self.nonce, self.timestamp, self.is_valid())
    }
}

#[cfg(test)]
mod test;