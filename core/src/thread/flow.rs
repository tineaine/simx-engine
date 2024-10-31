use crate::runtime::thread::get_engine_sender;
use engine_share::entity::common::SimxThreadSenderStringData;
use engine_share::entity::flow::flow::{FlowData, SubFlowTransferData};
use engine_share::entity::flow::node::Node;

pub fn exec_flow(path: String) -> Result<(), String> {
    let data = SimxThreadSenderStringData {
        function: "exec_flow".to_string(),
        data: path,
    };
    let sender = get_engine_sender("engine_sender");
    sender.unwrap().send(data).unwrap();
    Ok(())
}

#[allow(unused_variables)]
pub fn exec_steps(nodes: Vec<Node>, flow_data: FlowData) {
    let transfer = SubFlowTransferData {
        nodes,
        flow_data,
    };

    let data = SimxThreadSenderStringData {
        function: "exec_steps".to_string(),
        data: serde_json::to_string(&transfer).unwrap(),
    };
    let sender = get_engine_sender("engine_sender");
    sender.unwrap().send(data).unwrap();
}
