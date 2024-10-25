use crate::core::flow::controller::interface::check_require;
use crate::core::flow::dispatch::common::redress_stream_dispatch;
use crate::core::flow::dispatch::dispatch_general::dispatch_general;
use crate::core::flow::dispatch::dispatch_loop::dispatch_loop;
use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::exception::flow::flow_dispatch_err_handler;
use engine_common::logger::interface::{debug, fail};
use engine_common::runtime::flow::{get_flow_runtime, set_flow_runtime};
use engine_common::runtime::history::{history_persistent, log_history};
use engine_common::tools::common::{get_current_time, get_uuid};
use engine_share::entity::common::HistoryLog;
use engine_share::entity::exception::common::Status;
use engine_share::entity::exception::dispatch::DispatchErr;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::blueprint::Blueprint;
use engine_share::entity::flow::flow::{Flow, FlowData, FlowRuntimeModel, SystemFlowData};
use engine_share::entity::flow::node::{Node, NodeTag};
use std::path::Path;

// 流调度执行器
// 此方法会根据流文件的path，生成Flow运行时并调度执行
pub async fn dispatch_flow(path: &Path) -> Result<(), DispatchErr> {
    // 流对象
    let mut flow: Flow;

    // 在缓存中搜索路径是否存在，如果存在就不再走文件系统
    // TODO：这个判断并不稳定，有可能flow会发生改变，因此需要缓存文件修改日期进行比较，这个是后续的功能
    match get_flow_runtime(path.to_str().unwrap()) {
        Some(f) => {
            // 直接使用缓存中的数据
            flow = f;
        }
        None => {
            // 尝试解析流文件
            if path.exists() && path.is_file() {
                // 加载流文件并解析为Flow对象
                flow = flow_resolver(path);
                // 将当前流加入到缓存
                set_flow_runtime(path.to_str().unwrap(), flow.clone());
            } else {
                fail("cannot find or open flow file.");
                return Err(DispatchErr::FlowNotFound(path.to_str().unwrap().to_string()));
            }
        }
    }


    // 检查流文件的环境要求
    match check_require(flow.clone().requirements) {
        Ok(_) => {}
        Err(e) => {
            fail(e.as_str());
            return Err(DispatchErr::RequireError(e));
        }
    }

    let uuid = get_uuid();

    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        // status: FlowStatus::Starting,
        current_node: None,
        data: FlowData {
            // 系统数据表
            basics: SystemFlowData {
                flow_id: uuid.clone(),
                flow_name: "".to_string(),
                route: Default::default(),
                logs: vec![],
            },
            // 用户变量表
            params: Default::default(),
            // 节点Json数据
            json: Default::default(),
            // 节点二进制数据
            binary: Default::default(),
        },
    });

    debug(format!("flow {} :[{}] will be exec.", flow.name, uuid).as_str());

    let runtime = flow.clone().runtime.unwrap();
    // 取入口节点群并尝试执行
    let endpoints = flow.clone().blueprint.endpoints;
    for endpoint in endpoints {
        // 根据入口节点id获取节点对象
        let node = flow.blueprint.routes.get(&endpoint).expect("cannot find endpoint in router.");
        let mut node = node.clone();
        node.id = Some(endpoint.as_str().to_string());
        // 执行节点
        match dispatch_nodes(flow.blueprint.clone(), node, &mut runtime.data.clone()).await {
            Ok(_) => {}
            Err(e) => {
                return flow_dispatch_err_handler(e)
            }
        }
    }
    debug(format!("flow {} :[{}] has be exec success.", flow.name, uuid.clone()).as_str());
    // 将历史日志进行持久化
    history_persistent(uuid);
    Ok(())
}

// 调度节点（蓝图）
// 蓝图、首节点（从这个节点开始执行）、数据
pub async fn dispatch_nodes(blueprint: Blueprint, current_node: Node, data: &mut FlowData) -> Result<(), DispatchErr> {
    let node_uuid = get_uuid();
    let node_handler = current_node.clone().handler;
    let blueprint_id = current_node.clone().id.unwrap();
    let flow_id = data.clone().basics.flow_id.clone();
    let log_data = match current_node.clone().common {
        None => false,
        Some(comm) => {
            comm.log_data.unwrap_or_else(|| false)
        }
    };
    let data_logger: Option<FlowData>;
    if log_data {
        data_logger = Some(data.clone())
    } else {
        data_logger = None;
    }
    // 对通用配置进行操作
    node_common_handle(flow_id.clone(), node_uuid.clone(), node_handler.clone(), blueprint_id.clone(), Status::Start, "node will be exec.".to_string(), data_logger.clone());
    // 当前节点的标签列表
    let tags = match current_node.tags {
        None => { Vec::new() }
        Some(ref tags) => { tags.clone() }
    };
    // 当前节点是否为loop
    let is_loop = tags.contains(&NodeTag::Loop);
    // 是否为Jump节点
    let is_jump = tags.contains(&NodeTag::Jump);
    // Loop节点、Jump节点、Opt节点（操作节点）
    // Loop、Jump、Opt不参与exec_node调度，直接在此处实现
    if !current_node.handler.is_empty() {
        // 作为普通节点进行调度
        let ret = dispatch_general(blueprint, current_node, data).await;
        node_common_handle(flow_id.clone(), node_uuid.clone(), node_handler.clone(), blueprint_id.clone(), Status::End, "node exec successfully.".to_string(), data_logger);
        ret
    } else if is_loop {
        match Box::pin(dispatch_loop(blueprint.clone(), current_node.clone(), data)).await {
            Ok(_) => {
                node_common_handle(flow_id.clone(), node_uuid.clone(), node_handler.clone(), blueprint_id.clone(), Status::End, "node exec successfully.".to_string(), data_logger);
                Ok(())
            }
            Err(_) => {
                fail("The implicated compensation mechanism is triggered".to_string().as_str());
                node_common_handle(flow_id, node_uuid.clone(), node_handler.clone(), blueprint_id.clone(), Status::Fail, "The implicated compensation mechanism is triggered.".to_string(), data_logger);
                // 执行当前节点的Redress_stream，如果节点报错，会依次执行之前所有节点的Redress_stream
                return redress_stream_dispatch(NodeError::Redress("The implicated compensation mechanism is triggered".to_string()), &current_node, &blueprint, data).await;
            }
        }
    } else if is_jump {
        // 作为Jump节点进行处理
        Ok(())
    } else { Ok(()) }
}


fn node_common_handle(flow_id: String, node_id: String, handler: String, bp_id: String, status: Status, message: String, data: Option<FlowData>) {
    let history = &mut HistoryLog {
        handler,
        bp_id,
        node_id,
        status,
        snapshot: data.clone(),
        message,
        log_dt: get_current_time("%Y-%m-%d %H:%M:%S%.9f"),
    };

    log_history(flow_id, history.to_owned());
}