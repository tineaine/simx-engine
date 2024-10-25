use engine_common::logger::interface::warn;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::exception::node::NodeError::{HandleNotFound, ParamNotFound};
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub fn handle_core_var(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 创建/修改一个变量
        "set" => {
            // 判断用户也没有写进去变量数据，有可能编辑器没有拦截
            if node.attr.get("key").is_some() && node.attr.get("val").is_some() {
                let key = node.attr.get("key").unwrap().clone();
                let key = key.as_str().expect("Cannot convert key to str").to_string();
                let val = node.attr.get("val").unwrap().clone();
                flow_data.params.insert(key, val);
                Ok(())
            } else {
                warn("Cannot find variable name, Skip...");
                Err(ParamNotFound("key".to_string()))
            }
        }
        // 删除变量（使其失效）
        "remove" => {
            // 判断用户也没有写进去变量数据，有可能编辑器没有拦截
            if node.attr.get("var_name").is_some() {
                let key = node.attr.get("var_name").unwrap().clone();
                let key = key.as_str().expect("Cannot convert var_name to str");
                if flow_data.params.get(key).is_some() {
                    flow_data.params.remove(key);
                    Ok(())
                } else {
                    // 不拦截此错误
                    warn(format!("Cannot find variable by {}, Skip...", key).as_str());
                    Ok(())
                }
            } else {
                Err(ParamNotFound("var_name".to_string()))
            }
        }
        // 删除所有变量
        "remove_all" => {
            flow_data.params.clear();
            Ok(())
        }

        // 监听变量变化
        "watch" => {
            warn("Watch variable is not supported yet.");
            Ok(())
        }

        _ => {
            Err(HandleNotFound(node.handler))
        }
    }
}
