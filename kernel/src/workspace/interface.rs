use crate::workspace::project::load_project;
use engine_common::environment::interface::check;
use engine_common::logger::interface::{fail, info};
use engine_common::runtime::config::get_simx_config;
use engine_common::services::interface::load_service;
use engine_share::entity::workspace::Workspace;
use std::fs;
use std::path::PathBuf;

// 加载workspace到内存
pub async fn init_workspace() {
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
        // 仅对文件夹进行加载
        // 后续加载的其实是 swp,simx workspace package 包（zip）
        if path.is_dir() {
            // 在新的线程中初始化工作空间
            tokio::spawn(async move {
                load_workspace(path).await
            });
        }
    }
}

// 调起workspace
pub async fn load_workspace(path: PathBuf) {
    // TODO: 检查命名空间是否已经存在
    // 读取workspace的配置文件
    let workspace_conf_str = fs::read_to_string(path.join("workspace.ws")).expect("Cannot read workspace config file");
    let workspace_conf: Workspace = serde_json5::from_str(&workspace_conf_str).expect("Cannot parse workspace config file");
    // 初始化工作空间配置
    info("Load workspace config file successful");
    // 初始化工作空间变量
    info("Load workspace variable successful");
    // 检查环境需求
    match check(workspace_conf.global_requirement) {
        Ok(_) => {
            info("Workspace environment check successful");
        }
        Err(err) => {
            fail(format!("Workspace environment check failed: {}", err).as_str());
            return;
        }
    }
    // 加载服务
    for service in workspace_conf.global_service {
        // 加载服务
        load_service(service).await
    }
    // 加载项目
    for project in workspace_conf.module {
        load_project(path.join(project)).await
    }
    // 加载项目初始化脚本和流
    info(format!("Load workspace: {} successful", path.display()).as_str())
}