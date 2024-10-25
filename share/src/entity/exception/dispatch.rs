use crate::entity::exception::node::NodeError;

#[derive(Debug)]
pub enum DispatchErr {
    // 找不到流文件
    FlowNotFound(String),
    // 找不到节点
    NodeNotFound(String),
    // 运行超限（死循环异常）
    RunOverLimit,
    // 补偿流执行失败
    RedressFailed,
    // 流执行失败
    FlowFailed(String),
    // 流运行需求不满足
    RequireError(String),
    // 表达式执行失败
    EvalExprFailed(String),
    // 节点执行错误
    NodeRuntimeError(NodeError),
}