use bincode::config;
use engine_common::entity::ext::Transition;
use engine_common::logger::interface::warn;

mod handler;
mod tools;

#[no_mangle]
// 忽略编译器的警告
// 注意，此处不符合ffi，所以只能rust程序调用
#[allow(improper_ctypes_definitions)]
// 固定的入口函数，所有的调用都从此处开始
pub extern "C" fn interface(bytes: Vec<u8>) -> Vec<u8> {
    let transition: Transition = bincode::decode_from_slice(&bytes, config::standard()).expect("Cannot load info from bytes").0;
    // transition = handler(transition.clone());
    warn("hello world------ db");
    // warn(format!("{:?}", transition).as_str());
    let encoded: Vec<u8> = bincode::encode_to_vec(&transition, config::standard()).unwrap();
    encoded
}

#[no_mangle]
// 忽略编译器的警告
// 注意，此处不符合ffi，所以只能rust程序调用
#[allow(improper_ctypes_definitions)]
// 固定的入口函数，所有的调用都从此处开始
pub extern "C" fn init() {
    println!("init complete");
}