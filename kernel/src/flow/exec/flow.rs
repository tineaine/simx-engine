use crate::flow::exec::node::exec_node;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use serde_json::Value;

// 执行节点列表
// 拆分出来的目的是为了便于子节点的调用
pub async fn exec_steps(steps: Vec<Node>, mut flow_data: FlowData) -> Result<(), String> {
    // 计数器，计量当前在第几个节点
    let mut i = 0;

    // 流程节点执行
    Ok(for mut node in steps {
        i = i + 1;
        // 当前节点索引
        node.attr.insert("node_index".parse().unwrap(), Value::from(i.to_string()));
        // 将执行的结果保存到流对象中
        let ret = exec_node(node, &mut flow_data).await;
        if ret.is_err() {
            // 根据具体配置，处理节点中的报错
        }
    })
}