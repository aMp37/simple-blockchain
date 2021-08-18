use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::Serialize;

use super::util::{RSA2048DigitalSign, rsa_2048_encryption_provider::{RSA2048Provider, RSA2048Util}, sha_256_provider::{Sha256Hasher, Sha256Provider}};
#[derive(Debug)]
struct UserData<T> 
    where T: Serialize {
    content: T,
    author: Option<RsaPublicKey>,
    sign: Option<RSA2048DigitalSign>
}

impl<T> UserData<T> 
    where T: Serialize {

    pub fn new(content: T) -> Self {
        Self {
            content,
            author: None,
            sign: None
        }
    }

    pub fn sign_with_private_key(&mut self, author_private_key: &RsaPrivateKey) {
        let sign = UserData::generate_sign(author_private_key, &self.content);
        self.author = Some(RSA2048Util::get_public_key_from_private_key(author_private_key));
        self.sign = Some(sign);
    }

    fn generate_sign(private_key: &RsaPrivateKey, content_to_sign: &T) -> RSA2048DigitalSign {
        let data_to_hash = Self::serialize_content_to_json_string(content_to_sign);
        let content_hash = Sha256Hasher::hash_bytes(data_to_hash.as_bytes());
        RSA2048Util::digitally_sign_sha256(private_key, &content_hash)
    }

    fn serialize_content_to_json_string(content: &impl Serialize) -> String {
        serde_json::to_string(content).expect("Cannot Serialize content!")
    }

    fn validate_signed_vote(&self) -> bool {
        if self.is_signed() {
            let data_to_validate = Self::serialize_content_to_json_string(&self.content);
            let content_hash = Sha256Hasher::hash_bytes(data_to_validate.as_ref());
            return RSA2048Util::validate_signed(&content_hash, &self.sign.unwrap(), &self.author.as_ref().unwrap());
        }
        false
    }

    fn is_signed(&self) -> bool {
        self.author.is_some() && self.sign.is_some()
    }
}

#[cfg(test)]
mod test {
    use crate::blockchain::util::rsa_2048_encryption_provider::{RSA2048Provider, RSA2048Util};
    use super::UserData;
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    struct MockContent {
        c1: String,
        c2: i32
    }

    #[test]
    fn should_validate_return_true_when_correct_content() {
        let (_, mock_prv_key) = RSA2048Util::generate_rsa_keypair();
        let content=  MockContent {
            c1: String::from("352523"),
            c2: 53
        };

        let mut vote = UserData::new(content);
        vote.sign_with_private_key(&mock_prv_key);
        assert_eq!(vote.validate_signed_vote(), true);
    }

    #[test]
    fn should_validate_return_false_when_affected_content() {
        let (_, mock_prv_key) = RSA2048Util::generate_rsa_keypair();
        let content= MockContent {
            c1: String::from("352523"),
            c2: 53
        };
        let mut vote = UserData::new(content);
        vote.sign_with_private_key(&mock_prv_key);
        vote.content = MockContent {
            c1: String::from("2"),
            c2: 53
        };
        assert_eq!(vote.validate_signed_vote(), false);
    }
}