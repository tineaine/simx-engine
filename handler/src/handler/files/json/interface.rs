use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use serde_json::Value;
use std::fs;

pub fn handle_files_json(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 读取json文件
        "read" => read_as_json(node, flow_data),
        // 写json文件
        "write" => write_as_json(node),
        // 读取指定路径的值
        "get" => get_path(node, flow_data),
        // 设置指定路径的值（不存在就会新加）
        "set" => set_path(node, flow_data),
        // 删除目标路径
        "del" => read_as_json(node, flow_data),
        // 判断指定路径是否存在
        "exist" => read_as_json(node, flow_data),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}
fn read_as_json(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };

    match fs::read_to_string(path) {
        Ok(content) => {
            // 将读取到的数据尝试转换为Json对象
            let json_value: Value = match serde_json::from_str(&content) {
                Ok(json) => json,
                Err(err) => {
                    return Err(NodeError::FormatConvertError(err.to_string()));
                }
            };
            flow_data.json.insert(node.id.unwrap(), json_value);
            Ok(())
        }
        Err(err) => {
            Err(NodeError::FileReadError(err.to_string()))
        }
    }
}

fn write_as_json(node: Node) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };
    let content = match node.attr.get("content") {
        Some(value) => value,
        None => return Err(NodeError::ParamNotFound("content".to_string()))
    };

    match fs::write(path, content.as_str().unwrap().as_bytes()) {
        Ok(_) => {
            Ok(())
        }
        Err(err) => {
            return Err(NodeError::FileWriteError(err.to_string()))
        }
    }
}

fn get_path(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let node_id = node.id.unwrap();
    if let Some(json) = flow_data.json.get(&node_id) {
        let path = match node.attr.get("path") {
            Some(path) => path.as_str().unwrap(),
            None => return Err(NodeError::ParamNotFound("path".to_string()))
        };
        let value = json.pointer(path);
        if let Some(value) = value {
            flow_data.json.insert(node_id, value.clone());
        }
    }
    Ok(())
}

// 此功能需要优化，不好用 👎
fn set_path(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let new_value = match node.attr.get("value") {
        Some(value) => value,
        None => return Err(NodeError::ParamNotFound("value".to_string()))
    };
    let path = match node.attr.get("node_id") {
        Some(value) => value,
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };

    flow_data.json.insert(path.to_string(), new_value.clone());

    Ok(())
}
