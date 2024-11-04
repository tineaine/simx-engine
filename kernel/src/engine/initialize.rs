use crate::flow::interface::load_and_exec_default_flow;
use crate::script::interface::load_and_exec_default_script;
use crate::workspace::interface::init_workspace;
use engine_common::environment::check::env_check;
use engine_common::extension::interface::load_extension;
use engine_common::logger::interface::{fail, info, success, warn};
use engine_common::runtime::config::get_simx_config;
use engine_common::runtime::extension::set_extension_info;
use engine_common::runtime::flow::set_flow_info;
use engine_common::runtime::script::set_script_info;
use engine_share::entity::common::{SimxFlow, SimxScript};
use engine_share::entity::extension::Extension;
use serde_json::from_str;
use std::fs;
use std::path::Path;

pub async fn engine_init() -> Result<(), String> {
    // 系统引擎配置
    let engine_conf = get_simx_config().engine;


    // 检查工作环境（当前目录）
    match env_check() {
        Ok(_) => {
            info("Check env complete.")
        }
        Err(err) => {
            return Err(err);
        }
    }

    // 加载本地基础数据，包括蓝图、脚本、扩展
    if reload_local("").is_err() {
        fail("Cannot scan and load local resource")
    }

    // 初始化脚本
    if engine_conf.run_init_script {
        info("Default script running...");
        load_and_exec_default_script();
        success("Run init script complete.");
    }

    // 初始流
    if engine_conf.run_init_flow {
        info("Default flow running...");
        load_and_exec_default_flow().await;
        success("Run init flow complete.");
    }

    // 加载工作空间（项目集）
    info("Workspace initialization...");
    init_workspace().await;

    Ok(())
}

// 重新加载当前环境信息
// 比如当前系统中的脚本，流程等信息，这些信息会被加载到数据库中
pub fn reload_local(mode: &str) -> Result<String, String> {
    let engine_conf = get_simx_config().engine;
    // 这种写法虽然繁琐了点，但可以节省一小部分的内存...
    match mode {
        "script" => {
            let script_path = engine_conf.script_path;
            reload_local_traverse_folder(Path::new(script_path.as_str()), "script");
        }
        "flow" => {
            let flow_path = engine_conf.flow_path;
            reload_local_traverse_folder(Path::new(flow_path.as_str()), "flow");
        }
        "ext" => {
            let ext_path = engine_conf.ext_path;
            reload_local_traverse_folder(Path::new(ext_path.as_str()), "ext");
        }
        _ => {
            let ext_path = engine_conf.ext_path;
            let script_path = engine_conf.script_path;
            let flow_path = engine_conf.flow_path;
            // 加载脚本信息
            reload_local_traverse_folder(Path::new(script_path.as_str()), "script");
            // 加载流信息
            reload_local_traverse_folder(Path::new(flow_path.as_str()), "flow");
            // 加载插件信息
            reload_local_traverse_folder(Path::new(ext_path.as_str()), "ext");
        }
    }


    // 返回成功消息
    Ok("Scan complete.".to_string())
}

pub fn reload_local_traverse_folder(folder_path: &Path, traverse_type: &str) {
    let path_exist = Path::new(folder_path).is_dir();
    // 判断给定的路径是否存在
    if !path_exist {
        warn(format!("When initializing {}, folder {:?} not found, ignored err and skip.", traverse_type, folder_path).as_str());
        return;
    }

    if let Ok(entries) = fs::read_dir(folder_path) {
        // 循环指定的目录
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // 插件的信息会直接进入内存
                    if traverse_type.eq("ext") {
                        load_extension_by_path(path.as_path());
                    } else if traverse_type.eq("flow") {
                        load_flow_by_path(path.as_path());
                    } else if traverse_type.eq("script") {
                        if path.is_file() && !path.extension().is_none() {
                            load_script_by_path(path.as_path());
                        }
                    }
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    reload_local_traverse_folder(path.as_path(), traverse_type);
                }
            }
        }
    }
}

// 将指定路径下的插件信息加载到内存中
pub fn load_extension_by_path(path: &Path) {
    // 判断插件类型
    if path.exists() {
        if path.file_name().unwrap().to_str().unwrap().eq("extension.json") {
            // 读取 JSON 文件
            let file_path = Path::new(path);
            let data = fs::read_to_string(file_path).expect("Unable to read file");
            let mut extension: Extension = from_str(&data).expect("JSON was not well-formatted");
            extension.path = Some(file_path.parent().unwrap().to_str().unwrap().to_string());
            // 将数据放到 runtime 配置中
            set_extension_info(extension.name.as_str(), extension.clone());
            // 加载lib到内存
            load_extension(extension);
        }
    }
}

// 将路径下的流程信息加载到内存中
pub fn load_flow_by_path(path: &Path) {
    // 将数据放到 runtime 配置中
    set_flow_info(path.to_str().unwrap(), SimxFlow {
        id: 0,
        display_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        file_path: path.to_str().unwrap().to_string(),
        // 目前仅支持flow格式的蓝图
        file_type: "flow".to_string(),
        flow: None,
    });
}

// 将路径下的脚本信息加载到内存中
pub fn load_script_by_path(path: &Path) {
    // 将数据放到 runtime 配置中
    set_script_info(path.to_str().unwrap(), SimxScript {
        id: 0,
        display_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        file_path: path.to_str().unwrap().to_string(),
        file_type: path.extension().unwrap().to_str().unwrap().to_string(),
    });
}