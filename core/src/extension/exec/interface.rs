use crate::extension::core;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use std::env::consts::OS;

// 调用rust编写的扩展（直接是结构体）
pub fn call_exec(extension: Extension, node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    // 取方法所在插件文件名（相对于插件根目录）
    match OS.to_string().to_lowercase().as_str() {
        #[cfg(windows)]
        "windows" => core::call("exec", "dll", extension, None, Some(node), Some(flow_data)),
        #[cfg(unix)]
        "linux" => core::call("exec", "so", extension, None, Some(node), Some(flow_data)),
        #[cfg(unix)]
        "macos" => core::call("exec", "dylib", extension, None, Some(node), Some(flow_data)),
        _ => panic!("Not support this platform"),
    }
}