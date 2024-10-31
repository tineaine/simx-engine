use std::env;
use std::path::Path;
use std::process::Command;

use engine_common::logger::interface::{info, script_fail, script_log, warn};
use engine_common::runtime::config::get_simx_config;

pub fn exec_shell_script(path: &Path) {
    info(format!("Find Shell in path -> {:?}", path).as_str());
    let env_config = get_simx_config().env;
    // 获取操作系统类型
    let os = env::consts::OS;
    if os == "linux" || os == "macos" {
        if os == "macos" {
            if env_config.enable_shell_script_in_mac {
                warn("Run sh script on mac!")
            } else {
                info("Skip executing the sh script on mac.");
                return;
            }
        } else if os == "linux" {
            if !env_config.enable_shell_script_in_linux {
                info("Skip executing the sh script on linux.");
                return;
            }
        }
        let output = Command::new("sh")
            .arg(path)
            .output()
            .expect("Failed to execute script");
        // 输出执行结果
        if &*String::from_utf8_lossy(&output.stdout) != "" {
            script_log(&*String::from_utf8_lossy(&output.stdout).trim());
        }
        if &*String::from_utf8_lossy(&output.stderr) != "" {
            script_fail(&*String::from_utf8_lossy(&output.stderr).trim());
        }
    } else {
        info("Incompatible operating system, skip")
    }
}
