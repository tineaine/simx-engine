use crate::handler::core::debug::handle_core_debug;
use crate::handler::core::flow::handle_core_flow;
use crate::handler::core::service::interface::handle_core_service;
use crate::handler::core::variable::handle_core_var;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub async fn handle_core(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 调试
        "debug" =>
            handle_core_debug(node, flow_data),
        // 流控制
        "flow" =>
            handle_core_flow(node, flow_data).await,
        // 变量
        "var" =>
            handle_core_var(node, flow_data),
        // 服务控制
        "service" =>
            handle_core_service(node, flow_data).await,
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}
