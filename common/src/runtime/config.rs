use std::fs;
use std::path::Path;
use std::sync::Mutex;

use engine_share::entity::config::engine::{EngineConfig, EnvConfig, NetConfig, SimxConfig};
use once_cell::sync::Lazy;
// 用于存储运行时配置(Toml)
// 此处用于加载配置文件，并解析为结构体，注意配置一般不允许动态修改

// 全局静态变量
pub static CONFIG: Lazy<Mutex<SimxConfig>> = Lazy::new(|| {
    // 注意，此方法中不要调用日志打印相关方法，如果用户在没有默认日志文件夹的情况下运行，可能引起日志打印的报错
    // 判断是否有配置，现在允许无配置的情况下运行
    if !Path::new("conf").join("simx.toml").exists() {
        // 找不到配置，使用默认的配置运行
        let config: SimxConfig = SimxConfig {
            engine: EngineConfig::default(),
            net: NetConfig::default(),
            env: EnvConfig::default(),
        };
        Mutex::new(config)
    } else {
        // 发现配置，加载...
        let config_path = Path::new("conf").join("simx.toml");
        let config_str = fs::read_to_string(config_path).unwrap_or_else(|_| String::new());
        let config: SimxConfig = toml::from_str(&config_str).unwrap_or_default();
        Mutex::new(config)
    }
});

pub fn get_simx_config() -> SimxConfig {
    CONFIG.lock().unwrap().clone()
}