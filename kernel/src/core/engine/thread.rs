use crate::core::flow::exec::flow::exec_steps;
use crate::core::flow::interface::exec_flow;
use crate::core::script::interface::exec_script;
use engine_common::logger::interface::warn;
use engine_common::runtime::thread::set_engine_sender;
use engine_share::entity::common::SimxThreadSenderStringData;
use engine_share::entity::flow::flow::SubFlowTransferData;
use std::sync::mpsc;
use std::thread;
use tokio::runtime::Builder;

pub fn init_thread_monitor() {
    let rt = Builder::new_multi_thread()
        // 设置最大工作线程数为 8
        .worker_threads(8)
        // 设置最大阻塞线程数为 10
        .max_blocking_threads(10)
        // 启用所有 Tokio 的功能（如定时器、IO 等）
        .enable_all()
        .build()
        .unwrap();
    // 创建通道用于线程之间的消息传递
    let (engine_sender, engine_receiver) = mpsc::channel::<SimxThreadSenderStringData>();

    // 创建引擎监听线程
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
}