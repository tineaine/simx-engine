use crate::entity::flow::node::Node;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

// logical block
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Blueprint {
    pub parallel_endpoints: bool,
    pub parallel_routes: bool,
    pub endpoints: Vec<String>,
    pub routes: HashMap<String, Node>,
}