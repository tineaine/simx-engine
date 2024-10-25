use engine_common::logger::interface::{debug, fail, info, warn};
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub fn handle_core_debug(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        "node" => {
            // 普通文本
            node_debug(node, flow_data)
        },
        "print" => {
            match node.attr.get("text") {
                None => Err(NodeError::ParamNotFound("text".to_string())),
                Some(text) => {
                    // 这个方法不论传入了什么东西，都作为String方式输出
                    let level = match node.attr.get("level") {
                        None => "debug",
                        Some(level) => {
                            level.as_str().expect("level must be string")
                        }
                    };
                    match level {
                        "debug" => debug(format!("Node [{}] Print Output: {}", node.id.unwrap(), text.to_string()).as_str()),
                        "info" => info(format!("Node [{}] Print Output: {}", node.id.unwrap(), text.to_string()).as_str()),
                        "warn" => warn(format!("Node [{}] Print Output: {}", node.id.unwrap(), text.to_string()).as_str()),
                        "fail" => fail(format!("Node [{}] Print Output: {}", node.id.unwrap(), text.to_string()).as_str()),
                        // 就算用户乱填，也可以通过debug的方式打印
                        _ => debug(format!("Node [{}] Print Output: {}", node.id.unwrap(), text.to_string()).as_str()),
                    }
                    Ok(())
                }
            }
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 用于显示所有的节点内容，一般用于调试
fn node_debug(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    debug(format!("Debug Node: {:?}", node).as_str());
    debug(format!("Debug flow_data: {:?}", flow_data).as_str());
    Ok(())
}