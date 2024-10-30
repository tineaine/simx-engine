use crate::entity::common::HistoryLog;
use crate::entity::flow::blueprint::Blueprint;
use crate::entity::flow::node::Node;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Flow {
    // 蓝图版本
    pub version: String,
    // 流名称
    pub name: String,
    // 修改日期
    pub update_date: String,
    // 环境要求
    pub requirements: Vec<Environment>,
    // 执行蓝图
    pub blueprint: Blueprint,
    // 流运行时，此字段在调度器中赋值与管理
    pub runtime: Option<FlowRuntimeModel>,
}

// 流程状态模型
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowRuntimeModel {
    // 当前节点
    pub current_node: Option<Node>,
    // 流运行时数据
    pub data: FlowData,

}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvType {
    // 基本（引擎相关）
    Base,
    // 运行时（系统运行时，如Java、Python）
    Runtime,
    // 插件（引擎扩展插件）
    Plugin,
    // 扩展（功能扩展）
    Extend,
    // 运载服务（运行时服务，如rust、db等）
    Service,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Environment {
    pub name: String,
    pub env_type: EnvType,
    pub version: String,
}

// 流程数据
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FlowData {
    // 系统数据区，系统流转时用到的数据，不建议直接在节点中修改
    pub basics: SystemFlowData,
    // 用户变量区，用户可以在流中声明变量，存放于此处
    pub params: HashMap<String, Value>,
    // Json数据区，节点可以在此处存放Json数据对象
    pub json: HashMap<String, Value>,
    // 二进制数据区，节点可以在此处存放二进制的数据
    pub binary: HashMap<String, Vec<u8>>,
}

impl FlowData {
    pub fn get_basic(&self) -> SystemFlowData {
        self.basics.clone()
    }
    pub fn get_param(&self, key: &str) -> Option<&Value> {
        self.params.get(key)
    }

    pub fn set_param(&mut self, key: &str, value: Value) {
        self.params.insert(key.to_string(), value);
    }
    pub fn get_json(&self, key: &str) -> Option<&Value> {
        self.json.get(key)
    }

    pub fn set_json(&mut self, key: &str, value: Value) {
        self.json.insert(key.to_string(), value);
    }

    pub fn get_binary(&self, key: &str) -> Option<&Vec<u8>> {
        self.binary.get(key)
    }

    pub fn set_binary(&mut self, key: &str, value: Vec<u8>) {
        self.binary.insert(key.to_string(), value);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SystemFlowData {
    pub flow_id: String,
    pub flow_name: String,
    // 路由数据，一般由逻辑节点控制，用于控制节点的跳转
    pub route: HashMap<String, Vec<Value>>,
    // 日志数据
    pub logs: Vec<HistoryLog>,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubFlowTransferData {
    pub nodes: Vec<Node>,
    pub flow_data: FlowData,
}

// 历史日志步骤状态
#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum HistoryState {
    NodeStart,
    NodeFail,
    NodeWarn,
    NodeEnd,
}