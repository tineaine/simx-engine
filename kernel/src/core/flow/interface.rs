use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::core::flow::dispatch::interface::dispatch_flow;
use engine_common::logger::interface::info;
use engine_common::runtime::config::get_simx_config;
use engine_share::entity::exception::dispatch::DispatchErr;
use futures::future::BoxFuture;
use futures::FutureExt;

// 加载并执行默认流
pub async fn load_and_exec_default_flow() {
    let engine_conf = get_simx_config().engine;
    let flow_path = engine_conf.flow_path;
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new(flow_path.as_str()).join("init").is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(Path::new(flow_path.as_str()).join("init").as_path()).await;
    } else {
        info("No init flow found, Skip...")
    }
}

// 遍历并执行指定目录下的所有流（包含子目录）
fn traverse_folder(folder_path: &Path) -> BoxFuture<'static, ()> {
    let folder_path = Arc::new(folder_path.to_owned());
    let folder_path_clone = folder_path.clone();
    async move {
        if let Ok(entries) = fs::read_dir(&*folder_path_clone) {
            // Loop through the specified directory
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        exec_flow(&path).await.unwrap();
                    } else if path.is_dir() {
                        // If it's a directory, recursively traverse its contents
                        traverse_folder(&path).await;
                    }
                }
            }
        }
    }.boxed()
}

// 执行流
pub async fn exec_flow(path: &Path) -> Result<(), DispatchErr> {
    // 目前首选支持flow流程，其余都属于自定义的流类型
    // 目前其实flow也是json类型，但是后续flow可能会有加密之类的功能加上去
    // 系统不关系流的组织形式，只要能转化为标准流对象即可
    // exec_bp_flow(path).await
    // 调度执行
    dispatch_flow(path).await
}
