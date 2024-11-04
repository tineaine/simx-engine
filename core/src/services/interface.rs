use crate::extension::service::interface::call_server;
use engine_share::entity::services::Service;

pub async fn load_service(service: Service) {
    call_server(service).await.expect("cannot load service");
}

pub fn unload_service(service: Service) {
    println!("unload service {:?}", service);
}