#[cfg(windows)]
use crate::logger::interface::fail;
use crate::runtime::extension::get_extension_library;
#[cfg(windows)]
use engine_share::entity::extension::Extension;
use engine_share::entity::services::Service;
#[cfg(windows)]
use libloader::libloading::Symbol;
#[cfg(windows)]
use std::path::Path;
use engine_share::entity::exception::node::NodeError;

// 调用插件的初始化函数
#[cfg(windows)]
pub fn call_dll_extension_init(extension: Extension) -> Result<(), String> {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();

    // dll路径
    let dll_path = Path::new(&function_file).join(extension.entry_lib + ".dll");
    let dll_path = dll_path.as_os_str().to_str().unwrap();
    let lib = get_extension_library(dll_path).unwrap().win.unwrap();

    unsafe {
        let init: Symbol<unsafe extern "C" fn() -> bool> = lib.get("init".as_bytes()).expect("Could not find init function");
        // 调用函数
        if !init() {
            fail(format!("Call lib {} init failed ", extension.name).as_str())
        }
    }
    Ok(())
}

#[cfg(windows)]
pub fn call_dll_extension_service(extension: Extension, service: Service) -> Result<(), NodeError> {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();
    // dll路径
    let dll_path = Path::new(&function_file).join(extension.entry_lib + ".dll");
    let dll_path = dll_path.as_os_str().to_str().unwrap();

    let lib = get_extension_library(dll_path).unwrap().win.unwrap();

    unsafe {
        let serve: Symbol<unsafe extern "C" fn(Service) -> Result<(), NodeError>> = lib.get("serve".as_bytes()).expect("Could not find serve function");
        // 调用函数
        serve(service)
    }
}
