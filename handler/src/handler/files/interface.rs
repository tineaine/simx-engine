use crate::handler::files::dir::interface::handle_files_dir;
use crate::handler::files::file::interface::handle_files_file;
use crate::handler::files::json::interface::handle_files_json;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub fn handle_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 目录相关（列出目录、创建目录、删除目录、移动目录、重命名）
        "dir" => handle_files_dir(node, flow_data),
        // 文件相关（创建、重命名、删除、移动）
        "file" => handle_files_file(node, flow_data),
        // Json文件处理
        "json" => handle_files_json(node, flow_data),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}