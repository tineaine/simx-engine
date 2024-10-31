use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

// 通用的运行时数据

lazy_static! {
    static ref RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn set_runtime_info(key: &str, value: &str) {
    let mut data = RUNTIME.lock().unwrap();
    data.insert(key.to_string(), value.to_string());
}

pub fn get_runtime_info(key: &str) -> Option<String> {
    let data = RUNTIME.lock().unwrap();
    data.get(key).cloned()
}