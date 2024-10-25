use crate::entity::exception::node::NodeError;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    // 节点ID，在蓝图调度时赋值
    pub id: Option<String>,
    // 节点名称
    pub name: String,
    // 节点标签列表
    pub tags: Option<Vec<NodeTag>>,
    // 节点处理器路径，引擎会根据这个路径找到对应的handler
    pub handler: String,
    // 当前节点的配置
    pub attr: HashMap<String, Value>,
    // 当前节点的统一配置（共有的配置）
    pub common: Option<NodeCommonAttr>,
    // 下游节点id列表
    // 可以为以下的任意类型：
    // 1. id: String：直接指定下游节点id
    // 2. Map<expr: String，target: String>：指定下游节点id列表
    pub downstream: Vec<Value>,
    // 补偿流id列表
    pub redress_stream: Option<Vec<String>>,
}

impl Node {
    pub fn get_attr(&self, key: &str) -> Result<Value, NodeError> {
        match self.attr.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(NodeError::ParamNotFound(key.to_string())),
        }
    }

    pub fn has_tag(&self, tag: &NodeTag) -> bool {
        match self.tags {
            Some(ref tags) => tags.contains(tag),
            None => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeCommonAttr {
    // 异常处理机制
    pub exception_strategy: Option<NodeRedressType>,
    // 节点超时时间，单位：秒
    pub timeout: Option<i64>,
    // 重试次数
    pub retry_times: Option<i32>,
    // 重试间隔，单位：秒
    pub retry_interval: Option<f64>,
    pub log_data: Option<bool>,
    // 日志等级
    pub log_level: Option<String>,
    // 节点描述
    pub description: Option<String>,
    // 节点数据存储方式
    pub data_storage: Option<NodeDataStorage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeDataStorage {
    // 作为Json进行存储（如果可以转换）
    Json,
    // 作为二进制进行存储（不转换）
    Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeRedressType {
    // 跳过节点
    Skip,
    // 重试节点
    Retry,
    // 终止流运行
    Abort,
    // 节点补偿
    Redress,
    // 溯源补偿，相当于回溯之前所有的节点（默认）
    Trace,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeTag {
    // 计算节点，具有大量计算负荷
    Compute,
    // 命令节点，与操作系统进行命令交互
    Command,
    // 路由节点，会在节点执行结束后，要求调整执行路径
    Route,
    // 回环节点，此节点会重复执行其所有路由，直到路由失效
    Loop,
    // 跳跃节点
    Jump,
    // 数据节点，与数据库、数据文件进行交互
    Data,
    // 测试节点，仅用于调试和开发
    Debug,
    // 耗时节点，比如与第三方接口进行交互
    Delay,
    // 优先节点，会优先处理此节点
    Priority,
    // 阻塞节点，会阻塞调度器，直到被取消
    Sync,
    // 异步节点，不会等待此节点执行
    Async,
    // IO密集型节点
    IO,
}