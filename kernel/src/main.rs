use crate::core::engine::kernel::{run, serve};
use engine_common::logger::interface::info;
use engine_common::runtime::config::get_simx_config;
use std::env;
use std::fs;
use std::path::Path;

mod core;
#[tokio::main]
async fn main() {

    // 引擎运行前的准备和初始化动作
    init();
    // 分析用户输入参数，如果没有输入参数，就代表默认的启动方式
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 如果没有输入参数
    if args.len() > 1 {
        // 解析输入参数
        match args[1].as_str() {
            "serve" => serve().await,
            "run" => run().await,
            _ => run().await,
        }
        return;
    } else {
        // 同步运行监听
        serve().await;
    }
    // 程序运行结束后的清理动作
    // 注意，用户手动结束进程不会触发此方法
    clean();
}

// 初始化方法
fn init() {
    #[cfg(target_arch = "x86_64")]
    mimalloc_init();
    // 检查日志文件夹
    let engine_conf = get_simx_config().engine;
    // 检查运行目录下是否有日志目录
    let log_path = Path::new(engine_conf.log_path.as_str()).is_dir();
    if !log_path {
        // 重建日志目录
        fs::create_dir(engine_conf.log_path.as_str()).expect("Engine cannot fix workspace, Please check your environment.");
    }
}

// 这个是为了后续的内存池清理工作的准备
fn clean() {
    info("Simx engine run complete.");
}

// 针对于 x86_64 平台，使用 mimalloc 进行内存管理，可以大幅优化多线程内存效率
#[cfg(target_arch = "x86_64")]
fn mimalloc_init() {
    extern crate mimalloc_sys;

    use std::alloc::{GlobalAlloc, Layout};
    use std::ptr::null_mut;

    struct Mimalloc;

    unsafe impl GlobalAlloc for Mimalloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            mimalloc_sys::mi_malloc(layout.size()) as *mut u8
        }

        unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
            mimalloc_sys::mi_free(ptr as *mut std::ffi::c_void);
        }
    }

    #[global_allocator]
    static GLOBAL: Mimalloc = Mimalloc;
}