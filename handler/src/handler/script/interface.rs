use engine_common::thread::script::exec_script;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::node::Node;

pub fn handle_script(node: Node) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            match exec_script(path.to_string()) {
                Ok(_) => Ok(()),
                Err(e) => Err(NodeError::ScriptExecError(e))
            }
        }
        None => Err(NodeError::ParamNotFound("path".to_string())),
    }
}