// 此runtime主要由插件使用
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use lazy_static::lazy_static;
#[cfg(unix)]
use libloading::Library;

lazy_static! {
    static ref RUNTIME_EXTENSION: Mutex<HashMap<String, Extension>> = Mutex::new(HashMap::new());
    static ref RUNTIME_LIBRARY: Mutex<HashMap<String, ExtensionLibrary>> = Mutex::new(HashMap::new());
}

// 设置流信息，key是路径，value是流信息
pub fn set_extension_info(key: &str, value: Extension) {
    let mut data = RUNTIME_EXTENSION.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 获取流信息，key是路径，返回流信息
pub fn get_extension_info(key: &str) -> Option<Extension> {
    let data = RUNTIME_EXTENSION.lock().unwrap();
    data.get(key).cloned()
}

pub fn remove_extension_info(key: &str) {
    let mut data = RUNTIME_EXTENSION.lock().unwrap();
    data.remove(key);
}

// 获取所有流的信息，以数组的形式
pub fn get_all_extension_info() -> Vec<Extension> {
    let data = RUNTIME_EXTENSION.lock().unwrap();
    data.values().cloned().collect()
}

// 设置库
pub fn set_extension_library(key: &str, value: ExtensionLibrary) {
    let mut data = RUNTIME_LIBRARY.lock().unwrap();
    data.insert(key.to_string(), value);
}

pub fn get_extension_library(key: &str) -> Result<ExtensionLibrary, NodeError> {
    let data = RUNTIME_LIBRARY.lock().unwrap();
    Ok(data.get(key).cloned().unwrap())
}

pub fn remove_extension_library(key: &str) {
    let mut data = RUNTIME_LIBRARY.lock().unwrap();
    data.remove(key);
}

#[derive(Debug)]
pub struct ExtensionLibrary {
    #[cfg(windows)]
    pub win: Option<Arc<libloader::libloading::Library>>,
    #[cfg(unix)]
    pub linux: Option<Arc<Library>>,
    #[cfg(unix)]
    pub mac: Option<Arc<Library>>,
}

impl Clone for ExtensionLibrary {
    fn clone(&self) -> Self {
        ExtensionLibrary {
            #[cfg(windows)]
            win: self.win.clone(),
            #[cfg(unix)]
            linux: self.linux.clone(),
            #[cfg(unix)]
            mac: self.mac.clone(),
        }
    }
}