use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}

// 获取当前时间
pub fn get_current_time(format: &str) -> String {
    let now = chrono::Local::now();
    now.format(format).to_string()
}

pub fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap().as_nanos().to_string()
}