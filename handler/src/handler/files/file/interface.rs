use crate::handler::files::common::operation::{common_copy, common_exist, common_move, common_remove, read_str_file, write_str_file};
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::path::Path;

pub fn handle_files_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建文件
        "create" => make_file(node, flow_data),
        // 写文件（字符串）
        "write_str" => write_str_file(node),
        // 写文件（二进制）
        "write" => write_bin_file(node, flow_data),
        // 读文件（字符串）
        "read_str" => read_str_file(node, flow_data),
        // 读文件（二进制）
        "read" => read_bin_file(node, flow_data),
        // 判断文件是否存在
        "exist" => exist_file(node, flow_data),
        // 移动文件
        "mv" => mv_file(node),
        // 复制文件
        "cp" => cp_file(node),
        // 删除文件
        "del" => del_file(node),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 创建文件
fn make_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };

    let result = touch_file(path);
    if result.is_err() {
        result
    } else {
        // 如果执行成功，就将路径写入到节点数据域
        flow_data.json.insert(node.id.unwrap(), path.parse().unwrap());
        Ok(())
    }
}


// 读二进制
fn read_bin_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };
    let content_label = match node.attr.get("content_label") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("content_label".to_string()))
    };

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            flow_data.binary.insert(content_label.to_string(), buffer);
            Ok(())
        }
        Err(err) => {
            Err(NodeError::FileReadError(err.to_string()))
        }
    }
}


// 读二进制


fn write_bin_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };

    let content_label = match node.attr.get("content_label") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("content_label".to_string()))
    };

    let data = match flow_data.binary.get(content_label) {
        Some(data) => data,
        None => return Err(NodeError::FileWriteError("content label not found".to_string()))
    };

    let mut file = File::create(path).unwrap();
    match file.write_all(data) {
        Ok(_) => {}
        Err(_) => {
            return Err(NodeError::FileWriteError("write file failed".to_string()))
        }
    }
    Ok(())
}

fn touch_file(file_path: &str) -> Result<(), NodeError> {
    let path = Path::new(file_path);

    // 尝试打开文件，如果不存在则创建新文件
    let ret = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map(|_| ())
        .or_else(|err| {
            if err.kind() == io::ErrorKind::AlreadyExists {
                // 如果文件已存在，则直接打开文件
                File::open(&path).map(|_| ())
            } else {
                Err(err)
            }
        });
    if ret.is_err() {
        return Err(NodeError::FileCreateError);
    }
    Ok(())
}

fn mv_file(node: Node) -> Result<(), NodeError> {
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

fn del_file(node: Node) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            match common_remove(path.as_ref()) {
                Ok(_) => {
                    println!("File deleted successfully.{}", path);
                    Ok(())
                }
                Err(err) => {
                    println!("File deleted F.{:?}", err);
                    Err(err)
                }
            }
        }
        None => {
            Err(NodeError::ParamNotFound("path".to_string()))
        }
    }
}

fn cp_file(node: Node) -> Result<(), NodeError> {
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

fn exist_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
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
        None => {
            // 找不到参数
            Err(NodeError::ParamNotFound("path".to_string()))
        }
    }
}