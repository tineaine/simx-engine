use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    // 服务唯一标识，初始化与调用时需要使用
    pub service_key: String,
    // 插件唯一标识，初始化时会从其中获取服务
    pub extension_key: String,
    // 服务数据（json数据）
    pub data: Value,
}