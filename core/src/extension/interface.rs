use crate::extension::common::common::{call, get_extension_path};
use crate::logger::interface::{fail, info};
use crate::runtime::extension::{get_extension_info, remove_extension_info, remove_extension_library, set_extension_library, ExtensionLibrary};
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
#[cfg(windows)]
use libloader::libloading::Library;

#[cfg(unix)]
use engine_share::entity::services::Service;
#[cfg(unix)]
use libloading::Library;
use std::env::consts::OS;
use std::path::Path;
use std::sync::Arc;

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

// 调用rust编写的扩展（直接是结构体）
pub fn invoke_extension_func_common(extension: Extension, node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    // 取方法所在插件文件名（相对于插件根目录）
    match OS.to_string().to_lowercase().as_str() {
        #[cfg(windows)]
        "windows" => call("func", "dll", extension, None, Some(node), Some(flow_data)),
        #[cfg(unix)]
        "linux" => call("func", "so", extension, None, Some(node), Some(flow_data)),
        #[cfg(unix)]
        "macos" => call("func", "dylib", extension, None, Some(node), Some(flow_data)),
        _ => panic!("Not support this platform"),
    }
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
pub fn call_extension_init(extension: Extension) -> Result<(), NodeError> {
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

// 开启扩展中的某个服务
// 服务必须开启后才能通过处理器调用
pub async fn enable_extension_service(service: Service) -> Result<(), NodeError> {
    let extension: Vec<_> = service.extension_key.split(".").collect();
    let extension_name = extension[0];
    let extension: Extension = get_extension_info(extension_name).expect("Extension not found");
    info(format!("Try to call extension {} service", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    // 可能调用的与平台有关的库，比如dll、so、或dylib
    // 判断当前操作系统是windows、linux还是macos
    let job = tokio::spawn(async move {
        Ok(match OS.to_string().to_lowercase().as_str() {
            #[cfg(windows)]
            "windows" => {
                return call("serve", "dll", extension, Some(service), None, None)
            }
            #[cfg(unix)]
            "linux" => {
                return call("serve", "so", extension, Some(service), None, None)
            }
            #[cfg(unix)]
            "macos" => {
                return call("serve", "dylib", extension, Some(service), None, None)
            }
            _ => {}
        })
    });
    job.await.unwrap()?;
    // 将服务加入到服务列表中
    Ok(())
}