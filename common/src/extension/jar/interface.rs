use engine_share::entity::extension::Extension;
// use std::path::Path;
// 
// use crate::core::common::logger::interface::debug;
// 
// pub fn load_jar_extension(path: &Path) {
//     debug(format!("Load jar extension: {}", path.display()).as_str())
// }
// #[allow(unused_variables)]
// pub fn call_jar_extension_method(ext_path: String, node: Node, flow_data: &mut FlowData) -> FlowData {
//     flow_data.clone()
// }
#[allow(unused_variables)]
pub fn call_jar_extension_init(extension: Extension) -> Result<(), String> {
    Ok(())
}