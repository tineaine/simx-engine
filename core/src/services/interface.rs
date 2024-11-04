use crate::extension::interface::enable_extension_service;
use engine_share::entity::services::Service;

pub async fn load_service(service: Service) {
    enable_extension_service(service).await.expect("cannot load service");
}

pub fn unload_service(service: Service) {
    println!("unload service {:?}", service);
}