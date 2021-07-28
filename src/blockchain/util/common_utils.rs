use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_actual_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("illegal error").as_millis()
}
