use std::{convert::TryInto};

use sha2::{Digest, Sha256};

use super::Sha256Digest;

pub trait Sha256Provider {
    fn default_hasher() -> Self;
    fn update(&mut self, data: impl AsRef<[u8]>);
    fn finalize(self) -> Sha256Digest;
    fn hash_bytes(data: &[u8]) -> Sha256Digest;
}

pub struct Sha256Hasher {
    hasher: Sha256
}

impl Sha256Provider for Sha256Hasher {
    fn default_hasher() -> Self {
        Self {
            hasher: Sha256::default()
        }
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.hasher.update(data)
    }

    fn finalize(self) -> Sha256Digest {
        self.hasher.finalize().try_into().unwrap()
    }

    fn hash_bytes(data: &[u8]) -> Sha256Digest {
        let mut hasher = Self::default_hasher();
        hasher.update(data);
        hasher.finalize()
    }
}
