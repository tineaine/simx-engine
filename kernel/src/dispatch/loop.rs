use crate::dispatch::common::match_node_id;
use crate::dispatch::interface::dispatch_nodes;
use engine_common::expr::interface::expr_eval_bool;
use engine_common::logger::interface::{debug, fail};
use engine_share::entity::exception::dispatch::DispatchErr;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::blueprint::Blueprint;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub async fn dispatch_loop(blueprint: Blueprint, node: Node, flow_data: &mut FlowData) -> Result<(), DispatchErr> {
    let node_id = node.clone().id.expect("node id must be set");
    debug(format!("Loop start : {}", node_id).as_str());
    // 取开始列表
    let endpoints = node.attr.get("endpoints").expect("endpoints must be set").as_array().expect("endpoints must be array object");
    // 循环条件
    let expression = node.attr.get("expression").expect("expression must be set");
    // 取调度等待间隔
    let interval: f64 = match node.attr.get("interval") {
        None => {
            // 为-1就不等待，无间隙
            -1f64
        }
        Some(interval) => {
            interval.as_f64().unwrap()
        }
    };
    // 是否异步开始
    let parallel_endpoints: bool = match node.attr.get("parallel_endpoints") {
        None => {
            false
        }
        Some(parallel_endpoints) => {
            parallel_endpoints.as_bool().unwrap()
        }
    };
    // 取调度等待间隔
    let parallel_routes: bool = match node.attr.get("parallel_routes") {
        None => {
            // 为-1就不等待，无间隙
            false
        }
        Some(parallel_routes) => {
            parallel_routes.as_bool().unwrap()
        }
    };
    // 准备用于子流程的蓝图
    let mut blueprint: Blueprint = blueprint.clone();
    blueprint.endpoints.clear();
    blueprint.parallel_endpoints = parallel_endpoints;
    blueprint.parallel_routes = parallel_routes;

    let node_id = node.clone().id.expect("node id must be set");
    // 取目标路由数组
    // let router = node.attr.get("router").expect("cannot get expr").as_array().expect("router must be array object");
    // 取最大重试次数
    let loop_restriction: i64;
    // 首先判断node中有没有数据
    match flow_data.json.get(&node_id) {
        Some(value) => {
            loop_restriction = value.as_i64().unwrap();
        }
        None => {
            loop_restriction = match node.attr.get("maximum_repetition") {
                Some(value) => {
                    value.as_i64().unwrap()
                }
                None => {
                    // TODO：这个默认值应该在配置文件中写明
                    30
                }
            }
        }
    }


    // 执行表达式
    let mut result = match expr_eval_bool(expression.as_str().unwrap(),flow_data.clone().params) {
        Ok(value) => { value }
        Err(err) => {
            // 表达式执行出错
            return Err(DispatchErr::NodeRuntimeError(NodeError::ExpressionError(format!("Cannot parse expression {}, err: {}", expression, err))))
        }
    };
    // 循环计数器
    let mut index = 0;
    // 循环所有的开始节点
    for node_id in endpoints {
        let node = match match_node_id(node_id, &blueprint, flow_data.clone().params) {
            Ok(node) => { node }
            Err(err) => { return Err(err) }
        };

        // TODO：这个判断在并发下非常危险，临时使用
        while result && loop_restriction > index {
            debug(format!("Loop main struct start : {}, now in {}", node_id, index + 1).as_str());
            index += 1;
            // 执行表达式
            result = match expr_eval_bool(expression.as_str().unwrap(), flow_data.clone().params) {
                Ok(value) => { value }
                Err(err) => {
                    // 表达式执行出错
                    return Err(DispatchErr::NodeRuntimeError(NodeError::ExpressionError(format!("Cannot parse expression {}, err: {}", expression, err))))
                }
            };

            // 执行跳转逻辑
            dispatch_nodes(blueprint.clone(), node.clone(), flow_data).await?;
            // 执行等待间隙
            if interval != -1f64 {
                tokio::time::sleep(std::time::Duration::from_secs_f64(interval)).await;
            }
        }
    }
    if loop_restriction == index {
        fail("Loop run over limit, compulsory commutation!");
        return Err(DispatchErr::RunOverLimit);
    }
    debug(format!("Loop end : {}", node_id).as_str());
    Ok(())
}