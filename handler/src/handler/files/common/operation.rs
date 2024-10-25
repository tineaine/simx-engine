use engine_common::logger::interface::warn;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use serde_json::Value;
use std::fs::{metadata, rename};
use std::path::Path;
use std::{fs, io};

// 用于移动文件或文件夹
pub fn common_move(source: &str, target: &str, overwrite: bool) -> Result<(), NodeError> {
    let source_path = Path::new(source);
    let target_path = Path::new(target);

    // 检查源目录是否存在
    if !metadata(source_path).is_ok() {
        return Err(NodeError::PathNotFound);
    }

    // 检查目标位置是否已存在
    if metadata(target_path).is_ok() {
        if overwrite {
            // 强制模式下删除目标位置的内容
            match common_remove(target_path) {
                Ok(_) => {}
                Err(e) => { return Err(e) }
            }
        } else {
            // 警告即可，无需退出
            warn(format!("target dir {} exist, skip...", target).as_str())
        }
    }

    // 执行移动操作
    match rename(source_path, target_path) {
        Ok(_) => Ok(()),
        Err(e) => { Err(NodeError::PathMoveError(e.to_string())) }
    }
}

// 用于递归删除目录或文件
pub fn common_remove(path: &Path) -> Result<(), NodeError> {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            common_remove(&entry.path())?;
        }

        match fs::remove_dir(path) {
            Ok(_) => Ok(()),
            Err(_) => Err(NodeError::PathDeleteError)
        }
    } else if path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(_) => Err(NodeError::FileDeleteError)
        }
    } else {
        Err(NodeError::PathNotFound)
    }
}

// 用于递归复制文件或文件夹
pub fn common_copy(source_path: &Path, target_path: &Path) -> io::Result<()> {
    if source_path.is_dir() {
        // 如果源路径是一个目录，则递归复制目录及其内容
        fs::create_dir_all(target_path)?;
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                // 递归复制子目录
                common_copy(&entry.path(), &target_path.join(entry.file_name()))?;
            } else if file_type.is_file() {
                // 复制文件
                fs::copy(&entry.path(), &target_path.join(entry.file_name()))?;
            }
        }
    } else if source_path.is_file() {
        // 如果源路径是一个文件，直接复制文件
        fs::copy(source_path, target_path)?;
    }
    Ok(())
}

pub fn common_exist(path: &str) -> Result<bool, NodeError> {
    let path = Path::new(&path);
    // 检查目录是否存在
    if metadata(path).is_ok() {
        // 目录存在
        Ok(true)
    } else {
        // 目录不存在
        Ok(false)
    }
}

pub fn read_str_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };

    match fs::read_to_string(path) {
        Ok(content) => {
            flow_data.json.insert(node.id.unwrap(), Value::from(content));
            Ok(())
        }
        Err(err) => {
            Err(NodeError::FileReadError(err.to_string()))
        }
    }
}

pub fn write_str_file(node: Node) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };
    let content = match node.attr.get("content") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("content".to_string()))
    };

    match fs::write(path, content) {
        Ok(_) => {}
        Err(err) => {
            return Err(NodeError::FileWriteError(err.to_string()))
        }
    }
    Ok(())
}
