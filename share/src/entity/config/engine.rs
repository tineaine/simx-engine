use serde::Deserialize;
// 配置文件的实体类

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SimxConfig {
    pub engine: EngineConfig,
    pub net: NetConfig,
    pub env: EnvConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EngineConfig {
    // 最大线程数，默认为 100
    #[serde(default = "default_max_thread")]
    pub max_thread: u32,

    // 运行模式
    #[serde(default = "default_engine_mode")]
    pub engine_mode: String,

    // 在启动时运行默认脚本，默认为false
    #[serde(default = "default_run_init_script")]
    pub run_init_script: bool,

    // 在启动时运行默认流，默认为false
    #[serde(default = "default_run_init_flow")]
    pub run_init_flow: bool,

    // 系统启动后刷新本地数据
    #[serde(default = "default_auto_refresh_local_data")]
    pub auto_refresh_local_data: bool,

    // 扫描的流程目录
    #[serde(default = "default_flow_path")]
    pub flow_path: String,

    // 扫描的流程目录
    #[serde(default = "default_workspace_path")]
    pub workspace_path: String,

    // 扫描的脚本目录
    #[serde(default = "default_script_path")]
    pub script_path: String,

    // 扫描的插件目录
    #[serde(default = "default_ext_path")]
    pub ext_path: String,

    // 系统日志目录
    #[serde(default = "default_log_path")]
    pub log_path: String,

    // 控制台输出样式
    #[serde(default = "default_console_log_style")]
    pub console_log_style: bool,

    // 控制台日志等级
    #[serde(default = "default_shell_log_level")]
    pub shell_log_level: String,

    // 文件日志等级
    #[serde(default = "default_file_log_level")]
    pub file_log_level: String,

    // 缺少插件时的操作
    #[serde(default = "default_missing_plugin_action")]
    pub missing_plugin_action: String,

    // 错误的默认handel名称时的操作
    #[serde(default = "default_missing_default_handel_action")]
    pub missing_default_handel_action: String,

    // 错误的默认handel名称时的操作
    #[serde(default = "default_run_strategy")]
    pub run_strategy: String,

    // # 蓝图多入口并行，如果蓝图有多个入口，并行执行，默认为true
    #[serde(default = "default_blueprint_multi_entry_parallel")]
    pub blueprint_multi_entry_parallel: bool,

    // # 蓝图多下游并行，如果蓝图节点有多个下游，并行执行，默认为false
    #[serde(default = "default_blueprint_multi_downstream_parallel")]
    pub blueprint_multi_downstream_parallel: bool,
}

// 用于为字段提供默认值的实现
impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_thread: default_max_thread(),
            engine_mode: default_engine_mode(),
            run_init_script: default_run_init_script(),
            run_init_flow: default_run_init_flow(),
            auto_refresh_local_data: default_auto_refresh_local_data(),
            flow_path: default_flow_path(),
            workspace_path: default_workspace_path(),
            script_path: default_script_path(),
            ext_path: default_ext_path(),
            log_path: default_log_path(),
            console_log_style: default_console_log_style(),
            shell_log_level: default_shell_log_level(),
            file_log_level: default_file_log_level(),
            missing_plugin_action: default_missing_plugin_action(),
            missing_default_handel_action: default_missing_default_handel_action(),
            run_strategy: default_run_strategy(),
            blueprint_multi_entry_parallel: default_blueprint_multi_entry_parallel(),
            blueprint_multi_downstream_parallel: default_blueprint_multi_downstream_parallel(),
        }
    }
}

// 注意，默认值为20，仅用于测试或非大量数据的情况
fn default_max_thread() -> u32 {
    20
}

fn default_engine_mode() -> String {
    "memory".to_string()
}

fn default_run_init_script() -> bool {
    false
}

fn default_run_init_flow() -> bool {
    false
}

fn default_auto_refresh_local_data() -> bool {
    true
}

fn default_flow_path() -> String {
    "flow".to_string()
}

fn default_workspace_path() -> String {
    "workspace".to_string()
}

fn default_script_path() -> String {
    "script".to_string()
}

fn default_ext_path() -> String {
    "ext".to_string()
}

fn default_log_path() -> String {
    "logs".to_string()
}

fn default_console_log_style() -> bool {
    true
}

fn default_shell_log_level() -> String {
    "info".to_string()
}

fn default_file_log_level() -> String {
    "info".to_string()
}

fn default_missing_plugin_action() -> String {
    "warn".to_string()
}

fn default_missing_default_handel_action() -> String {
    "warn".to_string()
}

fn default_run_strategy() -> String {
    "serve".to_string()
}
fn default_blueprint_multi_entry_parallel() -> bool {
    true
}
fn default_blueprint_multi_downstream_parallel() -> bool {
    false
}

// 网络配置相关
#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    /// 是否启动 HTTP 监听
    #[serde(default = "default_rest_enable_listener")]
    pub rest_enable_listener: bool,

    /// 监听地址
    #[serde(default = "default_rest_listener_address")]
    pub rest_listener_address: String,

    /// 监听端口
    #[serde(default = "default_rest_listener_port")]
    pub rest_listener_port: u16,

    /// 工作线程数
    #[serde(default = "default_rest_listener_worker")]
    pub rest_listener_worker: usize,

    /// HTTP temp 临时目录所在位置
    #[serde(default = "default_rest_listener_temp_path")]
    pub rest_listener_temp_path: String,
}

impl Default for NetConfig {
    fn default() -> Self {
        Self {
            rest_enable_listener: default_rest_enable_listener(),
            rest_listener_address: default_rest_listener_address(),
            rest_listener_port: default_rest_listener_port(),
            rest_listener_worker: default_rest_listener_worker(),
            rest_listener_temp_path: default_rest_listener_temp_path(),
        }
    }
}

// 默认值函数
fn default_rest_enable_listener() -> bool {
    true
}

fn default_rest_listener_address() -> String {
    "127.0.0.1".to_string()
}

fn default_rest_listener_port() -> u16 {
    9898
}

fn default_rest_listener_worker() -> usize {
    5
}

fn default_rest_listener_temp_path() -> String {
    "tmp/web".to_string()
}

// 环境配置
#[derive(Debug, Deserialize, Clone)]
pub struct EnvConfig {
    /// 是否启用 Python 脚本
    #[serde(default = "default_enable_python_script")]
    pub enable_python_script: bool,

    /// 是否启用 Shell 脚本
    #[serde(default = "default_enable_shell_script")]
    pub enable_shell_script: bool,

    /// 是否启用 Batch 脚本
    #[serde(default = "default_enable_batch_script")]
    pub enable_batch_script: bool,

    /// 是否启用 PowerShell 脚本
    #[serde(default = "default_enable_powershell_script")]
    pub enable_powershell_script: bool,

    /// 是否在 macOS 上启用 Shell 脚本
    #[serde(default = "default_enable_shell_script_in_mac")]
    pub enable_shell_script_in_mac: bool,

    /// 是否在 Linux 上启用 Shell 脚本
    #[serde(default = "default_enable_shell_script_in_linux")]
    pub enable_shell_script_in_linux: bool,

    /// Python 解释器路径
    #[serde(default = "default_python_path")]
    pub python_path: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            enable_python_script: default_enable_python_script(),
            enable_shell_script: default_enable_shell_script(),
            enable_batch_script: default_enable_batch_script(),
            enable_powershell_script: default_enable_powershell_script(),
            enable_shell_script_in_mac: default_enable_shell_script_in_mac(),
            enable_shell_script_in_linux: default_enable_shell_script_in_linux(),
            python_path: default_python_path(),
        }
    }
}

// 默认值函数
fn default_enable_python_script() -> bool {
    true
}


fn default_enable_shell_script() -> bool {
    true
}

fn default_enable_batch_script() -> bool {
    true
}

fn default_enable_powershell_script() -> bool {
    true
}

fn default_enable_shell_script_in_mac() -> bool {
    true
}

fn default_enable_shell_script_in_linux() -> bool {
    true
}

fn default_python_path() -> String {
    "python3".to_string()
}
