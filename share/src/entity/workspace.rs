use crate::entity::flow::flow::Environment;
use crate::entity::services::Service;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub key: String,
    pub version: String,
    pub global_service: Vec<Service>,
    pub global_config: HashMap<String, Value>,
    pub global_variable: Vec<HashMap<String, Value>>,
    pub global_requirement: Vec<Environment>,
    pub module: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub key: String,
    pub path: Option<String>,
    pub version: String,
    pub service: Vec<Service>,
    pub config: HashMap<String, Value>,
    pub variable: Vec<HashMap<String, Value>>,
    pub requirement: Vec<Environment>,
    pub blueprint: Vec<String>,
}
