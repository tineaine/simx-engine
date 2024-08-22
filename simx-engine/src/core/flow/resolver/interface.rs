use crate::core::flow::resolver::flow::resolver_flow;
use engine_common::entity::flow::Flow;
use engine_common::logger::interface::warn;
use std::path::Path;

pub fn flow_resolver(path: &Path) -> Flow {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "flow" => resolver_flow(path),
            "json" => resolver_flow(path),
            "toml" => resolver_flow(path),
            "xml" => resolver_flow(path),
            // 目前拒绝处理其他脚本，直接返回空
            _ => {
                warn("Cannot resolver this extension file.");
                return Flow {
                    flow_name: "".to_string(),
                    update_date: "".to_string(),
                    create_date: "".to_string(),
                    developer: "".to_string(),
                    version: "".to_string(),
                    requirements: vec![],
                    steps: vec![],
                };
            }
        }
    } else {
        // 不解析其他任何后缀名的文件
        warn("Cannot resolver this extension file.");
        return Flow {
            flow_name: "".to_string(),
            update_date: "".to_string(),
            create_date: "".to_string(),
            developer: "".to_string(),
            version: "".to_string(),
            requirements: vec![],
            steps: vec![],
        };
    }
}