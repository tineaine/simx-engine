use engine_share::entity::common::SimxThreadSenderStringData;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{mpsc, Mutex};


lazy_static! {
    static ref THREAD: Mutex<HashMap<String, mpsc::Sender<SimxThreadSenderStringData>>> = Mutex::new(HashMap::new());
}

pub fn set_engine_sender(key: &str, value: mpsc::Sender<SimxThreadSenderStringData>) {
    let mut data = THREAD.lock().unwrap();
    data.insert(key.to_string(), value);
}

pub fn get_engine_sender(key: &str) -> Option<mpsc::Sender<SimxThreadSenderStringData>> {
    let data = THREAD.lock().unwrap();
    data.get(key).cloned()
}