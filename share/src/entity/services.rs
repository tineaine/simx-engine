use crate::entity::extension::Extension;

pub struct Service {
    // 服务运行时id（uuid）
    pub id: String,
    // 服务名称，调用时需要使用
    pub name: String,
    // 版本
    pub version: String,
    // 服务状态
    pub status: ServiceState,
    // 插件数据
    pub extension: Extension,
    // 服务数据（json数据）
    pub data: String,
}

pub struct ServiceState {
    // 是否启用
    pub enable: bool,
    // 使用者数量
    pub user_count: i32,
}

impl Service {
    // 创建
    pub fn new(id: String, name: String, version: String, extension: Extension) -> Service {
        Service {
            id,
            name,
            version,
            status: ServiceState {
                enable: false,
                user_count: 0,
            },
            extension,
            data: String::new(),
        }
    }

    pub fn is_enable(&self) -> bool {
        self.status.enable
    }

    pub fn enable(&mut self) {
        self.status.enable = true;
    }

    pub fn disable(&mut self) {
        self.status.enable = false;
    }
}