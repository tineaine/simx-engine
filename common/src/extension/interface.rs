use crate::extension::common::common::{common_call_method, get_extension_path};
use crate::extension::dll::interface::call_dll_extension_init;
use crate::extension::dylib::interface::call_dylib_extension_init;
use crate::extension::jar::interface::call_jar_extension_init;
use crate::extension::so::interface::call_so_extension_init;
use crate::logger::interface::{info, warn};
use crate::runtime::extension::{remove_extension_info, remove_extension_library, set_extension_library, ExtensionLibrary};
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use libloader::libloading::Library as WinLibrary;
use libloading::Library;
use std::env::consts::OS;
use std::path::Path;
use std::sync::Arc;

// 加载扩展
pub fn load_extension(extension: Extension) {
    let function_file = extension.path.as_ref().unwrap();
    let os = OS.to_string().to_lowercase();
    match os.as_str() {
        "windows" => {
            let path = Path::new(&function_file).join(extension.entry_lib + ".dll");
            println!("Load extension {:?}", path);
            let lib = unsafe { WinLibrary::new(path.clone()) }.expect("Could not load dll");
            set_extension_library(path.to_str().unwrap(), ExtensionLibrary {
                win: Some(Arc::new(lib)),
                linux: None,
                mac: None,
            });
        }
        "macos" => {
            let path = Path::new(&function_file).join(extension.entry_lib + ".dylib");
            let lib = unsafe { Library::new(path.clone()) }.expect("Could not load dylib");
            set_extension_library(path.to_str().unwrap(), ExtensionLibrary {
                win: None,
                linux: None,
                mac: Some(Arc::new(lib)),
            });
        }
        // 默认直接当so加载
        _ => {
            let path = Path::new(&function_file).join(extension.entry_lib + ".so");
            let lib = unsafe { Library::new(path.clone()) }.expect("Could not load so");
            set_extension_library(path.to_str().unwrap(), ExtensionLibrary {
                win: None,
                linux: Some(Arc::new(lib)),
                mac: None,
            });
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

// 调用rust编写的扩展（直接是结构体）
pub fn invoke_extension_func_common(extension: Extension, node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    // 取方法所在插件文件名（相对于插件根目录）
    let lib_path = get_extension_path(extension.path.unwrap(), extension.entry_lib);
    common_call_method(
        lib_path.to_str().unwrap(),
        OS.to_string().to_lowercase().as_str(),
        extension.handle_func.as_str(),
        node,
        flow_data,
    )
}

// 调用非rust编写的扩展（通过二进制或Json字符串）
pub fn invoke_extension_func_natural() {}

// 调用脚本接口
pub fn invoke_extension_func_script() {}

// 调用restful接口
pub fn invoke_extension_func_restful() {}

// 调用socket接口
pub fn invoke_extension_func_socket() {}

// 调用扩展的init
pub fn call_extension_init(extension: Extension) -> Result<(), String> {
    info(format!("Try to call extension {} init", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    let file_name = ext.entry_lib;
    if file_name.ends_with(".jar") {
        return call_jar_extension_init(extension);
    } else if file_name.ends_with(".py") {
        warn("Not support py file yet");
    } else {
        // 可能调用的与平台有关的库，比如dll、so、或dylib
        // 判断当前操作系统是windows、linux还是macos
        match OS.to_string().to_lowercase().as_str() {
            "windows" => {
                return call_dll_extension_init(extension);
            }
            "linux" => {
                return call_so_extension_init(extension);
            }
            "macos" => {
                return call_dylib_extension_init(extension)
            }
            _ => {}
        }
    }
    warn(format!("Function not found in extension {}", extension.name).as_str());
    Ok(())
}

// 开启扩展中的某个服务
pub fn enable_extension_service(extension: Extension) -> Result<(), String> {
    info(format!("Try to call extension {} init", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    let file_name = ext.entry_lib;
    if file_name.ends_with(".jar") {
        return call_jar_extension_init(extension);
    } else if file_name.ends_with(".py") {
        warn("Not support py file yet");
    } else {
        // 可能调用的与平台有关的库，比如dll、so、或dylib
        // 判断当前操作系统是windows、linux还是macos
        match OS.to_string().to_lowercase().as_str() {
            "windows" => {
                return call_dll_extension_init(extension);
            }
            "linux" => {
                return call_so_extension_init(extension);
            }
            "macos" => {
                return call_dylib_extension_init(extension)
            }
            _ => {}
        }
    }
    warn(format!("Function not found in extension {}", extension.name).as_str());
    Ok(())
}

// 禁用服务中的某个服务
pub fn disable_extension_init(extension: Extension) -> Result<(), String> {
    info(format!("Try to call extension {} init", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    let file_name = ext.entry_lib;
    if file_name.ends_with(".jar") {
        return call_jar_extension_init(extension);
    } else if file_name.ends_with(".py") {
        warn("Not support py file yet");
    } else {
        // 可能调用的与平台有关的库，比如dll、so、或dylib
        // 判断当前操作系统是windows、linux还是macos
        match OS.to_string().to_lowercase().as_str() {
            "windows" => {
                return call_dll_extension_init(extension);
            }
            "linux" => {
                return call_so_extension_init(extension);
            }
            "macos" => {
                return call_dylib_extension_init(extension)
            }
            _ => {}
        }
    }
    warn(format!("Function not found in extension {}", extension.name).as_str());
    Ok(())
}


// pub fn call_extension_destroy(extension: Extension) -> Result<(), String> {
//     Ok(())
// }