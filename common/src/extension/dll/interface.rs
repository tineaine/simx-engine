#[cfg(windows)]
use crate::logger::interface::fail;
#[cfg(windows)]
use engine_share::entity::extension::Extension;
#[cfg(windows)]
use libloader::libloading::{Library, Symbol};
#[cfg(windows)]
use std::path::Path;

// 调用插件的初始化函数
#[cfg(windows)]
pub fn call_dll_extension_init(extension: Extension) -> Result<(), String> {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();

    // dll路径
    let dll_path = Path::new(&function_file).join(extension.entry_lib + ".dll");
    let lib = unsafe { Library::new(dll_path) }.expect("Could not load dll");

    unsafe {
        let init: Symbol<unsafe extern "C" fn() -> bool> = lib.get("init".as_bytes()).expect("Could not find init function");
        // 调用函数
        if !init() {
            fail(format!("Call lib {} init failed ", extension.name).as_str())
        }
    }
    Ok(())
}
