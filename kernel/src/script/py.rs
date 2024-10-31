use std::path::Path;
use std::process::Command;

use engine_common::logger::interface::{info, script_fail, script_log, warn};
use engine_common::runtime::common::get_runtime_info;
use engine_common::runtime::config::get_simx_config;

pub fn exec_python_script(path: &Path) {
    let env_config = get_simx_config().env;
    let python = env_config.python_path;
    if !env_config.enable_python_script {
        return;
    }
    if get_runtime_info("env_python_status").unwrap().eq("not-find") {
        warn("Skip python script exec, cannot find python environment.");
        return;
    }
    // 文件名
    let file_name = path.file_name().unwrap().to_str().unwrap_or_default();
    info(format!("Find python in path -> {}", file_name).as_str());
    // 获取python的执行路径
    let output = Command::new(python)
        .arg(path)
        .output()
        .expect("Failed to execute py script");
    // 输出执行结果
    if &*String::from_utf8_lossy(&output.stdout) != "" {
        script_log(&*String::from_utf8_lossy(&output.stdout).trim());
    }
    if &*String::from_utf8_lossy(&output.stderr) != "" {
        script_fail(&*String::from_utf8_lossy(&output.stderr).trim());
    }
}
