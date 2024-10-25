use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Status {
    // 节点错误
    Fail,
    // 节点警告
    Warn,
    // 节点信息
    Info,
    // 节点调试
    Debug,
    // 节点开始
    Start,
    // 节点结束
    End,
    // 无状态（默认）
    #[default]
    None,
}