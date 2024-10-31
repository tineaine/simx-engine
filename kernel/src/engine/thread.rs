use crate::flow::exec::flow::exec_steps;
use crate::flow::interface::exec_flow;
use crate::script::interface::exec_script;
use engine_common::logger::interface::warn;
use engine_common::runtime::thread::set_engine_sender;
use engine_share::entity::common::SimxThreadSenderStringData;
use engine_share::entity::exception::engine::EngineErr;
use engine_share::entity::flow::flow::SubFlowTransferData;
use std::sync::mpsc;
use std::thread;
use tokio::runtime::Builder;

// 初始化线程监听
pub fn init_thread_monitor() -> Result<(), EngineErr> {
    let rt = Builder::new_multi_thread()
        // 最大工作线程
        .worker_threads(8)
        // 最大阻塞线程
        .max_blocking_threads(10)
        // 启用所有 Tokio 的功能（如定时器、IO 等）
        .enable_all()
        .build();
    let rt = match rt {
        Ok(rt) => rt,
        Err(_) => return Err(EngineErr::ThreadPoolInitErr("Cannot init thread pool.".to_string()))
    };
    // 线程通讯
    let (engine_sender, engine_receiver) = mpsc::channel::<SimxThreadSenderStringData>();

    // 引擎线程监听实现
    thread::spawn(move || {
        rt.block_on(async {
            for msg in engine_receiver {
                // 使用tokio的异步任务执行器来执行异步任务
                tokio::spawn(async move {
                    // 匹配线程消息对象
                    match msg.function.as_str() {
                        "exec_flow" => {
                            exec_flow(msg.data.as_ref()).await.expect("exec flow done with err");
                        }
                        "exec_script" => {
                            exec_script(msg.data.as_ref());
                        }
                        "exec_steps" => {
                            // 序列化出对象
                            let transfer = serde_json::from_str::<SubFlowTransferData>(msg.data.as_ref()).unwrap();
                            exec_steps(transfer.nodes, transfer.flow_data).await.expect("Cannot exec your flow.");
                        }
                        _ => {
                            warn("Unparsable thread message function!");
                        }
                    }
                });
            }
        });
    });

    set_engine_sender("engine_sender", engine_sender);
    Ok(())
}