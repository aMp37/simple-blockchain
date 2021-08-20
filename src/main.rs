extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_big_array;
use blockchain::{BlockChainDML, user_data::UserData};

mod blockchain;

fn main() {
    let mut blockchain = blockchain::BlockChain::new();
    blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    blockchain.insert_data(UserData::new(String::from("7903427897898973452")));
    blockchain.insert_data(UserData::new(String::from("7903427897898973452")));

    blockchain.print_blockchain();
    println!("Altering 2nd block!");
    unsafe {
        blockchain.alter_block_of_id(0, UserData::new(String::from("34534")));
    }
    blockchain.print_blockchain()
}
