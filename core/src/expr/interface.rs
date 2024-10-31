use evalexpr::{eval_boolean, EvalexprResult};

use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
// 计算布尔值
pub fn expr_eval_bool(value: &str, map: HashMap<String, Value>) -> EvalexprResult<bool> {
    let expr =  replace_placeholders(value, map);
    eval_boolean(expr.as_str())
}

fn replace_placeholders(input: &str, dist: HashMap<String, Value>) -> String {
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        match dist.get(&caps[1]) {
            Some(value) => value.as_str().unwrap(),
            None => "",
        }
    }).to_string()
}