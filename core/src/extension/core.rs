use crate::logger::interface::{fail, info};
use crate::runtime::extension::{remove_extension_info, remove_extension_library, set_extension_library, ExtensionLibrary};
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use engine_share::entity::services::Service;
#[cfg(windows)]
use libloader::libloading::Library;
#[cfg(windows)]
use libloader::libloading::Symbol;
#[cfg(unix)]
use libloading::Library;
#[cfg(unix)]
use libloading::Symbol;
use std::env::consts::OS;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

// 扩展调用方法，所有与扩展得交互均通过此方法实现（指Simx 扩展）
/**
func：方法名称，必须为 init、destroy、serve、func中的任意一个
lib_type：当前平台扩展类型，支持 dll、so、dylib
extension：当前插件信息
service：当前服务信息，可选，服务调用时必需，其余状态保持为None
node：当前节点信息，可选，方法调用时必需，其余状态保持为None
flow_data：当前流程数据，可选，方法调用时必需，其余状态保持为None
**/
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
            "destroy" => {
                let init: Symbol<unsafe extern "C" fn() -> bool> = lib.get("destroy".as_bytes()).expect("Could not find init function");
                // 调用函数
                if !init() {
                    fail(format!("Call lib {} init failed ", extension.name).as_str());
                    Err(NodeError::ExtError("Call extension init fail.".to_string()))
                } else {
                    Ok(())
                }
            }
            "serve" => {
                let serve: Symbol<unsafe extern "C" fn(service: Service) -> Result<(), NodeError>> = lib.get("serve".as_bytes()).expect("Call extension fail.");
                // 调用函数
                serve(service.unwrap())
            }
            "exec" => {
                let func: Symbol<unsafe extern "C" fn(Node, &mut FlowData) -> Result<(), NodeError>> = lib.get("exec".as_bytes()).expect("Could not find function");
                func(node.unwrap(), flow_data.unwrap())
            }
            // 匹配不到直接报错
            _ => {
                Err(NodeError::ExtError("Not support function name".to_string()))
            }
        }
    }
}

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

// 加载扩展
pub fn load_extension(extension: Extension) {
    let function_file = extension.path.as_ref().unwrap();
    let os = OS.to_string().to_lowercase();
    match os.as_str() {
        #[cfg(windows)]
        "windows" => {
            let path = Path::new(&function_file).join(extension.entry_lib + ".dll");
            println!("Load extension {:?}", path);
            let lib = unsafe { Library::new(path.clone()) }.expect("Could not load dll");
            set_extension_library(path.to_str().unwrap(), ExtensionLibrary {
                win: Some(Arc::new(lib)),
                #[cfg(unix)]
                linux: None,
                #[cfg(unix)]
                mac: None,
            });
        }
        #[cfg(unix)]
        "macos" => {
            let path = Path::new(&function_file).join(extension.entry_lib + ".dylib");
            let lib = unsafe { Library::new(path.clone()) }.expect("Could not load dylib");
            set_extension_library(path.to_str().unwrap(), ExtensionLibrary {
                #[cfg(windows)]
                win: None,
                linux: None,
                mac: Some(Arc::new(lib)),
            });
        }
        // 默认直接当so加载
        #[cfg(unix)]
        "linux" => {
            let path = Path::new(&function_file).join(extension.entry_lib + ".so");
            let lib = unsafe { Library::new(path.clone()) }.expect("Could not load so");
            set_extension_library(path.to_str().unwrap(), ExtensionLibrary {
                #[cfg(windows)]
                win: None,
                linux: Some(Arc::new(lib)),
                mac: None,
            });
        }
        _ => {
            fail("Platform not support");
        }
    };
}

// 卸载扩展
pub fn unload_extension(extension: Extension) {
    // 卸载掉插件信息和lib对象
    remove_extension_info(extension.name.as_str());
    let lib_path = get_extension_path(extension.path.unwrap(), extension.entry_lib);
    remove_extension_library(lib_path.to_str().unwrap());
}

// 调用扩展的init
pub fn call_init(extension: Extension) -> Result<(), NodeError> {
    info(format!("Try to call extension {} init", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    // 可能调用的与平台有关的库，比如dll、so、或dylib
    // 判断当前操作系统是windows、linux还是macos
    match OS.to_string().to_lowercase().as_str() {
        #[cfg(windows)]
        "windows" => {
            return call("init", "dll", extension, None, None, None)
        }
        #[cfg(unix)]
        "linux" => {
            return call("init", "so", extension, None, None, None)
        }
        #[cfg(unix)]
        "macos" => {
            return call("init", "dylib", extension, None, None, None)
        }
        _ => {}
    }
    Ok(())
}

