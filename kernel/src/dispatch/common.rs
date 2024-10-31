use crate::dispatch::interface::dispatch_nodes;
use engine_common::exception::node::node_expect_dispose;
use engine_common::expr::interface::expr_eval_bool;
use engine_share::entity::exception::dispatch::DispatchErr;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::blueprint::Blueprint;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use serde_json::Value;
use std::collections::HashMap;
use std::string::String;

// 流补偿机制
pub async fn redress_stream_dispatch(err: NodeError, current_node: &Node, blueprint: &Blueprint, data: &mut FlowData) -> Result<(), DispatchErr> {
    // 判断流是否可以继续执行
    if !node_expect_dispose(err) {
        return Err(DispatchErr::FlowFailed("Node execution failed".to_string()));
    }
    // 判断是否有补偿节点
    if current_node.redress_stream.is_some() {
        // 这部分需要根据配置进行，可以分线程或阻塞进行
        let redress_stream = current_node.redress_stream.clone().unwrap();
        for stream_id in redress_stream {
            let stream = blueprint.routes.get(&stream_id).expect("cannot find stream in router.");
            let mut stream = stream.clone();
            stream.id = Some(stream_id.as_str().to_string());
            // 尝试执行补偿流
            match Box::pin(dispatch_nodes(blueprint.clone(), stream.clone(), data)).await {
                Ok(_) => {}
                Err(_) => {
                    // 流执行失败
                    return Err(DispatchErr::RedressFailed);
                }
            }
        }
    }
    Ok(())
}

pub fn match_node_id(node_id: &Value, blueprint: &Blueprint, params: HashMap<String, Value>) -> Result<Node, DispatchErr> {
    let mut node: Node = Node {
        id: None,
        name: "".to_string(),
        tags: None,
        handler: "".to_string(),
        attr: Default::default(),
        common: None,
        downstream: vec![],
        redress_stream: None,
    };
    match node_id.is_string() {
        true => {
            let node_id = node_id.as_str().expect("Downstream id must be string");
            // 直接就是一个String，没有表达式
            node = blueprint.routes.get(node_id).expect("cannot find endpoint in router.").clone();
            node.id = Some(node_id.to_string());
            Ok(node)
        }
        false => {
            // 非string，需要进行表达式判断
            let downstream_expr = node_id.as_object().expect("Downstream expr must be object or string");
            let expr = downstream_expr.get("expr").expect("Downstream expr must have expr field").as_str().expect("Downstream expr expr must be string");
            match expr_eval_bool(expr, params) {
                Ok(result) => {
                    if !result {
                        // 相对于直接掉这个调度线，这个downstream不执行
                        return Ok(node);
                    }
                }
                Err(err) => { return Err(DispatchErr::EvalExprFailed(err.to_string())) }
            }
            let node_id = downstream_expr.get("target").expect("Downstream expr must have target field").as_str().expect("Downstream expr target must be string");
            node = blueprint.routes.get(node_id).expect("cannot find endpoint in router.").clone();
            node.id = Some(node_id.to_string());
            Ok(node)
        }
    }
}