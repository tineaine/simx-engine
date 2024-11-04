use engine_common::environment::interface::check;
use engine_common::logger::interface::{fail, info};
use engine_common::services::interface::load_service;
use engine_share::entity::workspace::Project;
use std::fs;
use std::path::PathBuf;

pub async fn load_project(path: PathBuf) {
    // TODO: 检查项目是否已经存在
    // 读取project的配置文件
    let project_conf_str = fs::read_to_string(path.join("project.ws")).expect("Cannot read project config file");
    let project_conf: Project = serde_json5::from_str(&project_conf_str).expect("Cannot parse project config file");
    // 初始化工作空间配置
    info("Load project config file successful");
    // 初始化工作空间变量
    info("Load project variable successful");
    // 检查环境需求
    match check(project_conf.requirement) {
        Ok(_) => {
            info("Project environment check successful");
        }
        Err(err) => {
            fail(format!("Project environment check failed: {}", err).as_str());
            return;
        }
    }
    // 加载服务
    for service in project_conf.service {
        // 加载服务
        load_service(service).await
    }
    // 加载蓝图
    for blueprint in project_conf.blueprint {
        // load_project(project)
        println!("load blueprint: {}", blueprint);
    }
    info(format!("Load project: {} successful", path.display()).as_str());
}