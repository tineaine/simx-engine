use crate::dispatch::common::{match_node_id, redress_stream_dispatch};
use crate::dispatch::interface::dispatch_nodes;
use crate::flow::exec::node::exec_node;
use engine_common::logger::interface::fail;
use engine_share::entity::exception::dispatch::DispatchErr;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::blueprint::Blueprint;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub async fn dispatch_general(blueprint: Blueprint, current_node: Node, mut data: &mut FlowData) -> Result<(), DispatchErr> {
    // 运行节点
    match exec_node(current_node.clone(), &mut data).await {
        Ok(_) => {}
        Err(e) => {
            redress_stream_dispatch(e, &current_node, &blueprint, data).await?
        }
    }

    if current_node.downstream.is_empty() {
        return Ok(());
    }
    // 下游节点处理
    Ok(for node_id in &current_node.downstream {
        let node = match match_node_id(node_id, &blueprint, data.clone().params) {
            Ok(node) => { node }
            Err(err) => { return Err(err) }
        };
        // 将递归调用的结果装箱
        // 调用下一个节点
        match Box::pin(dispatch_nodes(blueprint.clone(), node.clone(), data)).await {
            Ok(_) => {}
            Err(_) => {
                fail(format!("The implicated compensation mechanism is triggered, and the current node is {}", node_id).as_str());
                // 执行当前节点的Redress_stream，如果节点报错，会依次执行之前所有节点的Redress_stream
                return redress_stream_dispatch(NodeError::Redress(format!("The implicated compensation mechanism is triggered, and the current node is {}", node_id)), &current_node, &blueprint, data).await;
            }
        }
    })
}