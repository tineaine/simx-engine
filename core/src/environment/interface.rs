use crate::environment::java::check_java_version;
use engine_share::entity::flow::flow::Environment;

pub fn check(requirements: Vec<Environment>) -> Result<(), String> {
    for requirement in requirements {
        match requirement.name.as_str() {
            // Java 运行时，此处指JRE，同JRE
            "java" | "jre" => { check_java_version(); }
            // Jdk版本
            "jdk" => {}
            // 网络需求
            "net" => {}
            // Python 版本
            "python" => {}
            // simx 引擎版本
            "engine" => {}
            // 操作系统
            "os" => {}
            // Nodejs版本
            "nodejs" => {}
            // Npm 版本
            "npm" => {}
            // Python pip版本
            "pip" => {}
            // 操作系统lib库
            "lib" => {}
            // Handle扩展
            "ext" => {}
            // 引擎插件
            "plugin" => {}
            _ => {}
        }
    }
    Ok(())
}

