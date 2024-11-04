use engine::kernel::{run, serve};
use engine_common::logger::interface::info;
use engine_common::runtime::config::get_simx_config;
use std::env;
use std::fs;
use std::path::Path;

pub mod controller;
pub mod dispatch;
pub mod engine;
pub mod flow;
pub mod script;
mod workspace;

#[tokio::main]
async fn main() {

    // 引擎运行前的准备和初始化动作
    init();
    // 分析用户输入参数，如果没有输入参数，就代表默认的启动方式
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 如果没有输入参数
    if args.len() > 1 {
        // 解析输入参数
        match args[1].as_str() {
            "serve" => serve().await.unwrap(),
            "run" => run().await,
            _ => run().await,
        }
        return;
    } else {
        // 同步运行监听
        serve().await.unwrap();
    }
    // 程序运行结束后的清理动作
    // 注意，用户手动结束进程不会触发此方法
    clean();
}

// 初始化方法
fn init() {
    // 检查日志文件夹
    let engine_conf = get_simx_config().engine;
    // 检查运行目录下是否有日志目录
    let log_path = Path::new(engine_conf.log_path.as_str()).is_dir();
    if !log_path {
        // 重建日志目录
        fs::create_dir(engine_conf.log_path.as_str()).expect("Engine cannot fix workspace, Please check your environment.");
    }
}

// 这个是为了后续的内存池清理工作的准备
fn clean() {
    info("Simx engine run complete.");
}
