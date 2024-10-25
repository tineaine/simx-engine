use std::collections::HashMap;
use std::sync::Mutex;

use engine_share::entity::common::SimxFlow;
use engine_share::entity::flow::flow::Flow;
use lazy_static::lazy_static;

lazy_static! {
    // 流信息集合
    static ref FLOW_INFO: Mutex<HashMap<String, SimxFlow>> = Mutex::new(HashMap::new());
    // 运行时集合
    static ref RUNTIME_FLOW: Mutex<HashMap<String, Flow>> = Mutex::new(HashMap::new());
    // 流运行时状态
    static ref FLOW_RUNTIME_STATUS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 设置流信息，key是路径，value是流信息
pub fn set_flow_info(key: &str, value: SimxFlow) {
    let mut data = FLOW_INFO.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 设置流运行时
pub fn set_flow_runtime(key: &str, value: Flow) {
    // TODO：检查缓存大小
    let mut data = RUNTIME_FLOW.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 获取流运行时
pub fn get_flow_runtime(key: &str) -> Option<Flow> {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned()
}