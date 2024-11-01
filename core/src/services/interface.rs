use crate::extension::interface::enable_extension_service;
use engine_share::entity::services::Service;

pub fn load_service(service: Service) {
    println!("load service {:?}", service);
    enable_extension_service(service).expect("cannot load service");
}

pub fn unload_service(service: Service) {
    println!("unload service {:?}", service);
}