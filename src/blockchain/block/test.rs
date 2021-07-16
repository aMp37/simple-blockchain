use super::Block;


fn get_mock_block() -> Block {
    Block {
        block_id: 1,
        previous_block_hash: [0u8;32],
        data: String::from("MockData"),
        nonce: 23534,
        timestamp: 0u128
    }
}


#[test]
fn should_return_correct_hash_for_block() {
    println!("test {:?}", get_mock_block().calculate_hash());
}