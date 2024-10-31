use crate::logger::interface::info;
use crate::runtime::config::get_simx_config;
use crate::tools::common::get_timestamp;
use engine_share::entity::common::HistoryLog;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::fs::{metadata, File};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    // 流运转历史日志（这个日志一定小心，很容易把内存撑满）
    // key为流的flow id，val是该流的日志数组
    static ref FLOW_HISTORY: Mutex<HashMap<String, Vec<HistoryLog>>> = Mutex::new(HashMap::new());

}

pub fn log_history(flow_id: String, value: HistoryLog) {
    let mut data = FLOW_HISTORY.lock().unwrap();
    let mut arr: Vec<HistoryLog> = match data.get(flow_id.as_str()) {
        Some(arr) => arr.clone(),
        None => Vec::new(),
    };
    arr.push(value);
    data.insert(flow_id, arr);
}

pub fn history_persistent(flow_id: String) {
    let mut data = FLOW_HISTORY.lock().unwrap();
    let history = data.remove(flow_id.as_str()).expect("flow_id not found");
    // 获取日志路径
    let engine_conf = get_simx_config().engine;
    let log_path: &Path = engine_conf.log_path.as_ref();
    let path = log_path.join("history");
    if !metadata(path.clone()).is_ok() {
        match fs::create_dir(path.clone()) {
            Ok(_) => info("Create log path success..."),
            Err(_) => panic!("Create log path failed..."),
        }
    }
    let json = serde_json::to_string(&history).expect("fix struct failed");
    let mut file = File::create(path.join(format!("{}_{}.history", get_timestamp(), flow_id.as_str()))).unwrap();
    match file.write_all(json.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Create log path failed..."),
    }
}
