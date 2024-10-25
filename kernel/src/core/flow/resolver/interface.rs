use crate::core::flow::resolver::flow::resolver_flow;
use engine_common::logger::interface::warn;
use engine_share::entity::flow::flow::Flow;
use std::path::Path;

pub fn flow_resolver(path: &Path) -> Flow {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "bp" => resolver_flow(path),
            "json" => resolver_flow(path),
            // 目前拒绝处理其他脚本，直接返回空
            _ => {
                warn("Cannot resolver this extension file.");
                Flow::default()
            }
        }
    } else {
        // 不解析其他任何后缀名的文件
        warn("Cannot resolver this extension file.");
        Flow::default()
    }
}