#[derive(Debug)]
pub enum NodeError {
    // 找不到第三方的处理器
    ExtNotFound(String),
    // 插件内部错误
    ExtError(String),
    // handle执行错误
    HandleRuntimeError(String),
    // 找不到handle，需要附加找不到的handle
    HandleNotFound(String),
    // 路由错误
    RouteError(String),
    // 找不到必须的参数
    ParamNotFound(String),
    // 参数格式错误
    ParamFormatError(String),
    // 参数解析错误
    ParamParseError(String),
    // 找不到路径
    PathNotFound,
    // 创建目录失败
    PathCreateError,
    // 删除目录失败
    PathDeleteError,
    // 移动目录失败
    PathMoveError(String),
    // 复制目录失败
    PathCopyError,
    // 改变文件权限失败
    PathChmodError,
    // 其他目录错误
    PathOtherError(String),
    // 找不到文件
    FileNotFound,
    // 文件类型错误
    FileTypeError,
    // 文件读取错误
    FileReadError(String),
    // 文件写入错误
    FileWriteError(String),
    // 文件创建错误
    FileCreateError,
    // 文件删除错误
    FileDeleteError,
    // 文件移动/重命名错误
    FileMoveError,
    // 文件复制错误
    FileCopyError,
    // 文件权限设置错误
    FileChmodError,
    // 其他文件错误
    FileOtherError(String),
    // 格式转换错误
    FormatConvertError(String),
    // 需要管理员权限
    RequirePermission,
    // 脚本执行错误
    ScriptExecError(String),
    // 脚本执行超时
    ScriptExecTimeout,
    // 脚本执行失败
    ScriptExecFailed,
    // 脚本执行被拒绝
    ScriptExecRejected,
    // 找不到目标路径404
    NetworkUrlNotFound,
    // 网络未连接
    NetworkConnectError,
    // 网络请求错误
    NetworkRequestError,
    // 网络响应错误
    NetworkResponseError,
    // 网络请求超时
    NetworkTimeoutError,
    // 网络请求被拒绝
    NetworkRejectedError,
    // 其他网络错误
    NetworkOtherError(String),
    // 流程循环限制超额
    LoopLimitExceeded,
    // 循环执行错误
    LoopError(String),
    // 回滚异常（代表不是本节点的错误，是被其他节点牵连，会出现这个错误
    Redress(String),
    // 表达式执行错误
    ExpressionError(String),
}