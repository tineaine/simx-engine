use crate::core::environment::interface::check;
use engine_share::entity::flow::flow::Environment;

pub fn check_require(require: Vec<Environment>) -> Result<(), String> {
    check(require)
}