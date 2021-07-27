use rsa::{RSAPrivateKey, RSAPublicKey};

use super::util::{RSA2048DigitalSign, rsa_2048_encryption_provider::{RSA2048Provider, RSA2048Util}, sha_256_provider::{Sha256Hasher, Sha256Provider}};

#[derive(Debug)]
struct Vote {
    content:[u8;64],
    author: Option<RSAPublicKey>,
    sign: Option<RSA2048DigitalSign>
}

impl Vote {
    pub fn new(content: [u8;64]) -> Self {
        
        Self {
            content,
            author: None,
            sign: None
        }
    }

    pub fn sign_with_private_key(&mut self, author_private_key: &RSAPrivateKey) {
        let sign = Vote::generate_sign(author_private_key, &self.content);
        self.author = Some(RSA2048Util::get_public_key_from_private_key(author_private_key));
        self.sign = Some(sign);
    }

    fn generate_sign(private_key: &RSAPrivateKey, content_to_sign: &[u8;64]) -> RSA2048DigitalSign {
        let content_hash = Sha256Hasher::hash_bytes(content_to_sign);
        RSA2048Util::digitally_sign_sha256(private_key, &content_hash)
    }

    fn validate_signed_vote(&self) -> bool {
        if self.is_signed() {
            let content_hash = Sha256Hasher::hash_bytes(&self.content);
            return RSA2048Util::validate_signed(&content_hash, &self.sign.unwrap(), &self.author.as_ref().unwrap());
        }
        false
    }

    fn is_signed(&self) -> bool {
        self.author.is_some() && self.sign.is_some()
    }
}

#[cfg(test)]
mod test {use crate::blockchain::util::rsa_2048_encryption_provider::{RSA2048Provider, RSA2048Util};

    use super::Vote;

    #[test]
    fn should_validate_return_true_when_correct_content() {
        let (_, mock_prv_key) = RSA2048Util::generate_rsa_keypair();
        let content=  [23u8;64];
        let mut vote = Vote::new(content);
        vote.sign_with_private_key(&mock_prv_key);
        assert_eq!(vote.validate_signed_vote(), true);
    }

    #[test]
    fn should_validate_return_false_when_affected_content() {
        let (_, mock_prv_key) = RSA2048Util::generate_rsa_keypair();
        let content=  [23u8;64];
        let mut vote = Vote::new(content);
        vote.sign_with_private_key(&mock_prv_key);
        vote.content = [22u8;64];
        assert_eq!(vote.validate_signed_vote(), false);
    }
}