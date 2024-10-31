#[derive(Debug)]
pub enum EngineErr {
    // 引擎初始化失败
    EngineInitErr(String),
    // 内存池初始化失败
    MemoryPoolInitErr(String),
    // 内存管理初始化失败
    MemoryManagerInitErr(String),
    // 线程池初始化失败
    ThreadPoolInitErr(String),
    // 执行权限不足
    PermissionDenied(String),
    // 引擎运行失败（常规错误）
    EngineRunErr(String),
}