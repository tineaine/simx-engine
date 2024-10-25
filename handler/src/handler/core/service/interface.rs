use engine_share::entity::exception::node::NodeError;
use engine_share::entity::exception::node::NodeError::HandleNotFound;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub async fn handle_core_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 加载服务，加载并不意味着开启，只是将其放入了内存
        "load" => load_service(node, flow_data),
        // 卸载服务，卸载前会自动关闭服务
        "unload" => unload_service(node, flow_data),
        // 开启服务，开启服务，如果成功，check结果为true
        "enable" => enable_service(node, flow_data),
        // 关闭服务，关闭服务，如果成功，check结果为false
        "disable" => disable_service(node, flow_data),
        // 检查服务状态
        "check" => check_service(node, flow_data),
        // 调用服务，必须加载服务且服务处于开启状态
        "invoke" => invoke_service(node, flow_data),
        _ => {
            Err(HandleNotFound(node.handler))
        }
    }
}

fn load_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

fn unload_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}
fn enable_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

fn disable_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

fn check_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

fn invoke_service(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

