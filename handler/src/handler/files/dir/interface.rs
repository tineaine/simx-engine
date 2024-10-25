use crate::handler::files::common::operation::{common_copy, common_exist, common_move, common_remove};
use engine_common::logger::interface::info;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use serde_json::Value;
use std::fs;
use std::fs::metadata;
use std::path::Path;

pub fn handle_files_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建目录
        "create" => create_dir(node, flow_data),
        // 判断目录是否存在
        "exist" => exist_dir(node, flow_data),
        // 移动目录
        "mv" => mv_dir(node),
        // 复制目录
        "cp" => cp_dir(node),
        // 删除目录
        "del" => del_dir(node),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 创建目录
pub fn create_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.get_attr("path") {
        Ok(path) => {
            let path = path.as_str().expect("path must be string");
            let path = Path::new(&path);
            // 检查目录是否存在
            if metadata(path).is_ok() {
                info(format!("Path {} exist, skip...", path.display()).as_str());

                Ok(())
            } else {
                // 创建目录
                match fs::create_dir(path) {
                    Ok(_) => {
                        // 写到节点数据域
                        flow_data.json.insert(node.id.unwrap(), Value::from(path.display().to_string()));
                        Ok(())
                    }
                    // 目录创建失败
                    Err(_) => Err(NodeError::PathCreateError)
                }
            }
        }
        Err(e) => Err(e)
    }
}

// 判断指定文件夹是否存在
pub fn exist_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.get_attr("path") {
        Ok(path) => {
            let path = path.as_str().expect("path must be string");
            // 检查目录是否存在
            if common_exist(path).expect("Cannot check path exist") {
                // 目录存在
                flow_data.json.insert(node.id.unwrap(), Value::from(true));
                Ok(())
            } else {
                // 目录不存在
                flow_data.json.insert(node.id.unwrap(), Value::from(false));
                Ok(())
            }
        }
        Err(e) => Err(e)
    }
}

// 移动目录到新位置
pub fn mv_dir(node: Node) -> Result<(), NodeError> {
    let source_path = match node.attr.get("source") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("source".to_string())),
    };

    let target_path = match node.attr.get("target") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("target".to_string())),
    };

    let source_path = source_path.as_str().expect("source must be string");
    let target_path = target_path.as_str().expect("target must be string");


    match common_move(source_path, target_path, true) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// 复制目录到新位置
pub fn cp_dir(node: Node) -> Result<(), NodeError> {
    let source_path = match node.attr.get("source") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("source".to_string())),
    };

    let target_path = match node.attr.get("target") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("target".to_string())),
    };

    let source_path: &Path = source_path.as_str().expect("source must be string").as_ref();
    let target_path: &Path = target_path.as_str().expect("target must be string").as_ref();

    common_copy(source_path, target_path).expect("Cannot cp dir");
    Ok(())
}

// 删除目录
pub fn del_dir(node: Node) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            match common_remove(path.as_ref()) {
                Ok(_) => Ok(()),
                Err(err) => Err(err)
            }
        }
        None => {
            Err(NodeError::ParamNotFound("path".to_string()))
        }
    }
}
