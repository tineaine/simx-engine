use tklog::{debug, error, info, warn};

pub fn info(text: &str) {
    info!(text);
}

pub fn warn(text: &str) {
    warn!(text);
}

pub fn fail(text: &str) {
    error!(text);
}

pub fn success(text: &str) {
    info!(text);
}

pub fn script_log(text: &str) {
    info!(text);
}

pub fn script_fail(text: &str) {
    info!(text);
}

pub fn debug(text: &str) {
    debug!(text);
}
