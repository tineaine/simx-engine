use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub filename: String,
    pub description: String,
    #[serde(rename = "isAsync")]
    pub is_async: bool,
    pub params: Vec<FunctionParam>,
    pub result: FunctionResult,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Extension {
    pub path: Option<String>,
    pub name: String,
    pub version: String,
    pub engine: String,
    pub author: String,
    pub dependencies: Vec<String>,
    pub entry_lib: String,
    pub init: String,
    pub destroy: String,
    pub handle_func: String,
    pub handle_service: String,
}

// #[derive(Debug)]
// pub struct ExtensionLibrary {
//     pub win: Option<Arc<libloader::libloading::Library>>,
//     pub linux: Option<Arc<Library>>,
//     pub mac: Option<Arc<Library>>,
// }

// impl Clone for ExtensionLibrary {
//     fn clone(&self) -> Self {
//         ExtensionLibrary {
//             win: self.win.clone(),
//             linux: self.linux.clone(),
//             mac: self.mac.clone(),
//         }
//     }
// }