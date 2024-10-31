use crate::logger::interface::fail;
use engine_share::entity::exception::node::NodeError;

// 节点异常统一处理
// 如果返回了false，将断开流的执行
pub fn node_expect_dispose(node_err: NodeError) -> bool {
    match node_err {
        NodeError::ExtNotFound(ext) => {
            fail(format!("extension {} could not be found.", ext).as_str());
            return false;
        }
        // 扩展中的方法执行失败
        NodeError::ExtError(ext) => {
            fail(format!("extension {} method execution failed.", ext).as_str());
            return false;
        }
        NodeError::HandleRuntimeError(_) => {}
        NodeError::HandleNotFound(handler) => {
            fail(format!("handler {} could not be found.", handler).as_str());
            return false;
        }
        NodeError::RouteError(_) => {}
        NodeError::ParamNotFound(_) => {}
        NodeError::ParamFormatError(_) => {}
        NodeError::ParamParseError(_) => {}
        NodeError::PathNotFound => {}
        NodeError::PathCreateError => {}
        NodeError::PathDeleteError => {}
        NodeError::PathMoveError(_) => {}
        NodeError::PathCopyError => {}
        NodeError::PathChmodError => {}
        NodeError::PathOtherError(_) => {}
        NodeError::FileNotFound => {}
        NodeError::FileTypeError => {}
        NodeError::FileReadError(_) => {}
        NodeError::FileWriteError(_) => {}
        NodeError::FileCreateError => {}
        NodeError::FileDeleteError => {}
        NodeError::FileMoveError => {}
        NodeError::FileCopyError => {}
        NodeError::FileChmodError => {}
        NodeError::FileOtherError(_) => {}
        NodeError::RequirePermission => {}
        NodeError::ScriptExecError(_) => {}
        NodeError::ScriptExecTimeout => {}
        NodeError::ScriptExecFailed => {}
        NodeError::ScriptExecRejected => {}
        NodeError::NetworkUrlNotFound => {}
        NodeError::NetworkConnectError => {}
        NodeError::NetworkRequestError => {}
        NodeError::NetworkResponseError => {}
        NodeError::NetworkTimeoutError => {}
        NodeError::NetworkRejectedError => {}
        NodeError::NetworkOtherError(_) => {}
        NodeError::LoopLimitExceeded => {}
        NodeError::LoopError(_) => todo!(),
        NodeError::Redress(_) => {},
        NodeError::ExpressionError(_) => todo!(),
        NodeError::FormatConvertError(_) => {}
    }
    true
}

