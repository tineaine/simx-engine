use engine_common::thread::flow::{exec_flow, exec_steps};
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::exception::node::NodeError::{HandleNotFound, HandleRuntimeError};
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub async fn handle_core_flow(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 子流程
        "sub_flow" => sub_flow(node, flow_data),
        // 发起流程
        "post_flow" => {
            match node.attr.get("flow_path") {
                Some(path) => {
                    let path = path.as_str().expect("flow_path is not string");
                    match exec_flow(path.to_string()) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(HandleRuntimeError(e))
                    }
                }
                None => Err(NodeError::ParamFormatError("flow_path is none".to_string()))
            }
        },
        // 流等待
        "timeout" => {
            // 等待一段时间
            // tokio::
            match node.attr.get("sec") {
                Some(sec) => {
                    tokio::time::sleep(std::time::Duration::from_secs_f64(sec.as_f64().unwrap())).await;
                    // TODO: 等待流程执行完毕，并返回结果
                    Ok(())
                }
                None => Err(NodeError::ParamFormatError("flow_id is none".to_string()))
            }
        },
        _ => {
            Err(HandleNotFound(node.handler))
        }
    }
}

pub fn sub_flow(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.attr.get("steps") {
        Some(steps) => {
            let nodes_str = steps.as_str().expect("steps is not string");
            // 拆分nodes
            let nodes: Vec<Node> = match serde_json::from_str::<Vec<Node>>(nodes_str) {
                Ok(nodes) => nodes,
                Err(e) => {
                    return Err(NodeError::ParamFormatError(e.to_string()));
                }
            };
            let sub_flow_data = flow_data.clone();
            exec_steps(nodes, sub_flow_data);
            Ok(())
        }
        None => {
            Err(NodeError::ParamNotFound("steps".to_string()))
        },
    }
}