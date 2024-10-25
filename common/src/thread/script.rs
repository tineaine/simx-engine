use crate::runtime::thread::get_engine_sender;
use engine_share::entity::common::SimxThreadSenderStringData;

pub fn exec_script(path: String) -> Result<(), String> {
    let data = SimxThreadSenderStringData {
        function: "exec_script".to_string(),
        data: path,
    };
    let sender = get_engine_sender("engine_sender");
    sender.unwrap().send(data).unwrap();
    Ok(())
}