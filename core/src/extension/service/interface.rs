use crate::extension::core;
use crate::logger::interface::info;
use crate::runtime::extension::get_extension_info;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::services::Service;
use std::env::consts::OS;

// 开启扩展中的某个服务
// 服务必须开启后才能通过处理器调用
pub async fn call_server(service: Service) -> Result<(), NodeError> {
    let extension: Vec<_> = service.extension_key.split(".").collect();
    let extension_name = extension[0];
    let extension: Extension = get_extension_info(extension_name).expect("Extension not found");
    info(format!("Try to call extension {} service", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    // 可能调用的与平台有关的库，比如dll、so、或dylib
    // 判断当前操作系统是windows、linux还是macos
    let job = tokio::spawn(async move {
        Ok(match OS.to_string().to_lowercase().as_str() {
            #[cfg(windows)]
            "windows" => {
                return core::call("serve", "dll", extension, Some(service), None, None)
            }
            #[cfg(unix)]
            "linux" => {
                return core::call("serve", "so", extension, Some(service), None, None)
            }
            #[cfg(unix)]
            "macos" => {
                return core::call("serve", "dylib", extension, Some(service), None, None)
            }
            _ => {}
        })
    });
    job.await.unwrap()?;
    // 将服务加入到服务列表中
    Ok(())
}