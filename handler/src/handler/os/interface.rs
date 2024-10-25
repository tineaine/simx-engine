use crate::handler::os::shell::handle_os_shell_println;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub fn handle_os(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 执行shell命令
        "shell" => handle_os_shell_println(node, flow_data),
        // 获取所有进程
        "process" => { Ok(()) }
        // 查询指定进程是否存活
        "alive" => { Ok(()) }
        // 启动指定进程
        "start" => { Ok(()) }
        // 停止指定进程
        "kill" => { Ok(()) }
        // 获取操作系统信息
        "info" => { Ok(()) }
        // 锁定系统
        "lock" => { Ok(()) }
        // 进入睡眠模式
        "sleep" => { Ok(()) }
        // 注销当前用户
        "logout" => { Ok(()) }
        // 重启系统
        "reboot" => { Ok(()) }
        // 关闭系统
        "shutdown" => { Ok(()) }
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}
