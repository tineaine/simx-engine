use crate::logger::interface::fail;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use engine_share::entity::services::Service;
#[cfg(windows)]
use libloader::libloading::Symbol;
#[cfg(unix)]
use libloading::Library;
#[cfg(windows)]
use libloader::libloading::Library;
#[cfg(unix)]
use libloading::Symbol;
use std::env::consts::OS;
use std::path::{Path, PathBuf};


// 组装插件的真实路径
pub fn get_extension_path(path: String, entry_lib: String) -> PathBuf {
    let os = OS.to_string().to_lowercase();
    match os.as_str() {
        "windows" => {
            Path::new(&path).join(entry_lib + ".dll")
        }
        "linux" => {
            Path::new(&path).join(entry_lib + ".so")
        }
        "macos" => {
            Path::new(&path).join(entry_lib + ".dylib")
        }
        _ => {
            Path::new(&path).join(entry_lib + ".so")
        }
    }
}

pub fn call(func: &str, lib_type: &str, extension: Extension, service: Option<Service>, node: Option<Node>, flow_data: Option<&mut FlowData>) -> Result<(), NodeError> {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();
    // lib路径
    let lib_path = Path::new(&function_file).join(extension.entry_lib + "." + lib_type);
    let lib = unsafe { Library::new(lib_path) }.expect("Could not load lib");

    unsafe {
        match func {
            "init" => {
                let init: Symbol<unsafe extern "C" fn() -> bool> = lib.get("init".as_bytes()).expect("Could not find init function");
                // 调用函数
                if !init() {
                    fail(format!("Call lib {} init failed ", extension.name).as_str());
                    Err(NodeError::ExtError("Call extension init fail.".to_string()))
                } else {
                    Ok(())
                }
            }
            "serve" => {
                let serve: Symbol<unsafe extern "C" fn(service: Service) -> Result<(), NodeError>> = lib.get(func.as_bytes()).expect("Call extension fail.");
                // 调用函数
                serve(service.unwrap())
            }
            "func" => {
                let func: Symbol<unsafe extern "C" fn(Node, &mut FlowData) -> Result<(), NodeError>> = lib.get(func.as_ref()).expect("Could not find function");
                func(node.unwrap(), flow_data.unwrap())
            }
            // 匹配不到直接报错
            _ => {
                Err(NodeError::ExtError("Not support function name".to_string()))
            }
        }
    }
}
