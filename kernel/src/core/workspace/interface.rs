use crate::core::environment::interface::check;
use engine_common::logger::interface::fail;
use engine_common::runtime::config::get_simx_config;
use engine_share::entity::flow::flow::Environment;
use std::fs;
use std::path::PathBuf;

// 加载workspace到内存
pub fn init_workspace() {
    // 系统引擎配置
    let engine_conf = get_simx_config().engine;
    let workspace_path = engine_conf.workspace_path;
    // 获取项目路径
    let project_path = workspace_path;
    // 获取项目路径下的所有文件
    let entries = fs::read_dir(project_path).unwrap();
    // 遍历文件
    for entry in entries {
        // 获取文件路径
        let path = entry.unwrap().path();
        println!("{:?}", path);
        // 仅对文件夹进行加载
        if path.is_dir() {
            // 作为项目加载到内存中
            // TODO：改为多进程方式
            load_workspace(path)
        }
    }
}

// 调起workspace
pub fn load_workspace(path: PathBuf) {
    // 加载项目环境需求并判断是否满足需求
    let requirements: Vec<Environment> = vec![];
    match check(requirements) {
        Ok(_) => {}
        Err(e) => {
            fail(format!("Cannot load workspace, request env not match: {}", e).as_str());
        }
    }
    // 加载项目服务
    // 加载项目初始化脚本和流
    println!("Load workspace: {}", path.display())
}