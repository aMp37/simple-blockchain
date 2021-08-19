extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_big_array;


use blockchain::BlockChainDML;

mod blockchain;

fn main() {
    let mut blockchain = blockchain::BlockChain::new();
    blockchain.insert_data(String::from("7903427897898973452"));
    blockchain.insert_data(String::from("7903427897898973452"));
    blockchain.insert_data(String::from("7903427897898973452"));
    blockchain.insert_data(String::from("7903427897898973452"));
    blockchain.insert_data(String::from("7903427897898973452"));

    blockchain.print_blockchain();
    println!("Altering 2nd block!");
    blockchain.alter_block_of_id(4, String::from("34534"));
    blockchain.print_blockchain()
}
