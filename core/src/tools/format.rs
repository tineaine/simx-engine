use engine_share::entity::exception::node::NodeError;

// 将Json转换为u8进行存储
pub fn json_to_u8(json: String) -> Vec<u8> {
    json.into_bytes()
}

// 将u8转换为Json
pub fn u8_to_json(u8: Vec<u8>) -> String {
    String::from_utf8(u8).unwrap()
}

// 将u8转换为字符串
pub fn u8_to_str(u8: Vec<u8>) -> String {
    String::from_utf8(u8).unwrap()
}

// 将字符串转换为u8
pub fn str_to_u8(str: String) -> Vec<u8> {
    str.into_bytes()
}

// 此方法用于取参数的值，如果为空就报错
pub fn opt_u8_to_str(u8: Option<&Vec<u8>>, param: String) -> Result<String, NodeError> {
    match u8 {
        Some(path) => Ok(u8_to_str(path.clone())),
        None => Err(NodeError::ParamNotFound(param))
    }
}