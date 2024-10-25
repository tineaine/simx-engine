use crate::entity::exception::common::Status;
use crate::entity::flow::flow::{Flow, FlowData};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxFlow {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    // 流对象，加载时机是执行流前
    pub flow: Option<Flow>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxScript {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
}


pub struct SimxThreadSenderStringData {
    pub function: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HistoryLog {
    // 节点handler
    pub handler: String,
    // 蓝图节点id
    pub bp_id: String,
    // 节点id（运行时id）
    pub node_id: String,
    // 当前状态
    pub status: Status,
    // 记录时间
    pub log_dt: String,
    // 数据快照
    pub snapshot: Option<FlowData>,
    // 日志消息
    pub message: String,
}