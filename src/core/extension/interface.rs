use std::fs;
use std::path::Path;

use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::{fail, info};
use crate::core::extension::dll::interface::load_dll_extension;
use crate::core::extension::jar::interface::load_jar_extension;
use crate::core::extension::py::interface::load_py_extension;
use crate::core::extension::so::interface::load_so_extension;

// 加载并执行默认脚本
pub fn load_local_extendion() {
    info("Load Extension...");
    // debug(format!("Ext Path: {}", get_runtime_conf("ext_path").unwrap()).as_str());
    // engine_conf.get("engine").unwrap().get("run-init-script").unwrap().as_bool().unwrap()
    let script_path = get_runtime_conf("ext_path").unwrap();
    // TODO: 将这个路径修改到配置文件中
    let binding = Path::new(script_path.as_str()).iter().as_path();
    let path = binding;
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new(path).is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(path);
    } else {
        info("No extensions found, Skip...")
    }
}

// 遍历并执行指定目录下的所有脚本（包含子目录）
fn traverse_folder(folder_path: &Path) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        // 循环指定的目录
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // 是一个文件，尝试作为脚本进行解析
                    load_extension_by_path(path.as_path())
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    traverse_folder(path.as_path());
                }
            }
        }
    }
}

// 此方法根据一个url加载插件（相当于从远程下载）
// pub fn load_extension_by_url() {}

// 此方法根据路径加载插件，注意是通过文件后缀名判断插件类型，因此必须有正确地插件名称
pub fn load_extension_by_path(path: &Path) {
    // 判断插件类型
    if path.exists() {
        if let Some(extension) = path.extension() {
            // 交给对应的加载程序
            match extension.to_str().unwrap().to_lowercase().as_str() {
                "jar" => load_jar_extension(path),
                "dll" => load_dll_extension(path),
                "so" => load_so_extension(path),
                "py" => load_py_extension(path),
                // 没有后缀名的文件统一作为so加载
                _ => load_so_extension(path)
            }
        } else {
            fail("Cannot find assign extension file path.")
        }
    }
}
//
// // 检查插件可用性
// pub fn check_extension_available() {}
//
// // 获取指定插件信息
// pub fn get_extension_info() {}
//
// // 获取所有插件信息
// pub fn get_all_extension_info() {}
//
// // 获取指定插件的详细信息
// pub fn get_extension_detail() {}
//
// // 卸载指定插件
// pub fn unload_extension() {}
//
// // 删除指定插件
// pub fn delete_extension() {}
//
// // 获取指定插件功能列表
// pub fn get_extension_func(){
//
// }
//
// // 执行目标插件的目标功能
// pub fn execute_extension_func(){
//
// }