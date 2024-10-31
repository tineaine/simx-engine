use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RUNTIME_ENGINE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn set_engine_info(key: &str, value: &str) {
    let mut data = RUNTIME_ENGINE.lock().unwrap();
    data.insert(key.to_string(), value.to_string());
}

// pub fn get_engine_info(key: &str) -> Option<String> {
//     let data = RUNTIME_ENGINE.lock().unwrap();
//     data.get(key).cloned()
// }