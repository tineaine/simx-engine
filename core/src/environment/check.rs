use crate::environment::python::check_python;
use crate::logger::interface::{info, success, warn};
use crate::runtime::common::set_runtime_info;
use crate::runtime::config::get_simx_config;
use std::fs;
use std::path::Path;

/// 环境检查
pub fn env_check() -> Result<String, String> {
    let env_config = get_simx_config().env;

    info("Checking the operating environment...");

    // 检查运行目录下是否有缓存目录
    let tmp_path = Path::new("tmp").is_dir();
    if !tmp_path {
        info("Cannot find tmp dir, system will automatically rebuild this directory.");
        // 重建缓存目录
        // 这个目录下会保存临时内容，包括引擎运行时产生的临时数据和Rest服务运行时的缓存文件
        fs::create_dir("tmp").expect("Engine cannot fix workspace, Please check your environment.");
    }


    // 检查是否有 Python 环境
    if !check_python(env_config.python_path) {
        warn("Env Check: Python not found.");
        set_runtime_info("env_python_status", "not-find");
    } else {
        set_runtime_info("env_python_status", "python3")
    }

    success("Checking the operating environment complete.");
    Ok("Check done.".to_string())
}