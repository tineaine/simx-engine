use std::env;
use std::path::Path;
use std::process::Command;

use engine_common::logger::interface::{info, script_fail, script_log};
use engine_common::runtime::config::get_simx_config;

pub fn exec_powershell_script(path: &Path) {
    let env_config = get_simx_config().env;
    info(format!("Find powershell in path -> {:?}", path).as_str());
    if !env_config.enable_powershell_script {
        info("Powershell script disabled, skip");
        return;
    }
    // 获取操作系统类型
    let os = env::consts::OS;
    // 仅windows 执行
    if os == "windows" {
        let output = Command::new("powershell")
            .arg("-File")
            .arg(path)
            .output()
            .expect("Failed to execute script");
        // 输出执行结果
        if String::from_utf8_lossy(&output.stdout) != "" {
            script_log(String::from_utf8_lossy(&output.stdout).trim());
        }
        if String::from_utf8_lossy(&output.stderr) != "" {
            script_fail(String::from_utf8_lossy(&output.stderr).trim());
        }
    } else {
        info("Incompatible operating system, skip")
    }
}
