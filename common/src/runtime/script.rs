use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

use engine_share::entity::common::SimxScript;

lazy_static! {
    static ref RUNTIME_SCRIPT: Mutex<HashMap<String, SimxScript>> = Mutex::new(HashMap::new());
}

// 设置流信息，key是路径，value是流信息
pub fn set_script_info(key: &str, value: SimxScript) {
    let mut data = RUNTIME_SCRIPT.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 获取流信息，key是路径，返回流信息
pub fn get_script_info(key: &str) -> Option<SimxScript> {
    let data = RUNTIME_SCRIPT.lock().unwrap();
    data.get(key).cloned()
}

// 获取所有流的信息，以数组的形式
pub fn get_all_script_info() -> Vec<SimxScript> {
    let data = RUNTIME_SCRIPT.lock().unwrap();
    data.values().cloned().collect()
}