// 项目配置文件
{
  // 项目唯一标识，在工作空间中必须唯一，但不同工作空间中允许重复
  key: 'crm-system-api',
  // 项目名称，仅设计器属性
  name: 'CRM 系统服务',
  // 项目描述，仅设计器属性
  description: 'CRM 核心系统服务',
  // 项目版本号
  version: '1.0.0',
  // 项目作者，仅设计器属性
  author: 'NJ Labs',
  // 更新时间
  update_date: '2024-11-01 14:00:00',
  // 仅对当前项目可访问
  service: [],
  // 项目配置，如有重复，会覆盖引擎、工作空间的配置
  config: {
    // 当蓝图文件、文件夹发生变动后，主动重新加载项目
    auto_reload: true,
    // 是否启用日志记录
    log: true
  },
  // 项目全局变量
  variable: [
  ],
  // 项目需求列表
  requirement: [
  ],
  // 蓝图
  blueprint: [
    ""
  ]
}