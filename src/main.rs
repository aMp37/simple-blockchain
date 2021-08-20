extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_big_array;
use blockchain::{BlockChainDML, user_data::UserData};

use crate::blockchain::{Signable, util::{self, rsa_2048_encryption_provider::RSA2048Provider}};

mod blockchain;

fn main() {

    let (_, prv_key) = util::rsa_2048_encryption_provider::RSA2048Util::generate_rsa_keypair();

    let mut data = UserData::new(String::from("7903427897898973452"));
    data.sign_with_private_key(&prv_key);
    let mut blockchain = blockchain::BlockChain::new();
    blockchain.insert_data(data);
    // blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    // blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    // blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    // blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    println!("blockchain validity: {}", blockchain.validate_blockchain())
}
