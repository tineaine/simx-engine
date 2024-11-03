// 工作空间配置
{
  // 工作空间唯一标识
  key: 'nj-crm',
  // 工作空间名称
  // 设计器属性，引擎不会读取此属性
  name: 'CRM 项目组',
  // 工作空间配置版本
  version: '1.0.0',
  // // 设计器属性，引擎不会读取此属性
  description: 'NJ Labs CRM 工作空间',
  // 更新时间
  update: '2024-11-01 00:00:00',
  // 全局服务配置（这里启用的服务可以被所有项目读取和使用，比如一些restful服务可以在此处启动，但项目特有的数据库连接池则适合在单独的项目中控制）
  global_service: [
    {
      // 服务唯一标识，会在服务启动时使用此key进行初始化，这是工作空间的服务，因此引擎中全局禁止重复（包括其他工作空间）
      service_key: 'crm-http-service',
      // 插件标识（引擎会根据此标识去插件中调起服务，第一个字符串是插件名称，第二个字符串是插件提供的服务名称，插件必须已经加载到系统中）
      extension_key: "simx-http.server",
      // 交予服务的配置，这是一个json字符串，可以直接写对象
      data: {
        // 获取监听地址
        addr: "127.0.0.1",
        // 获取监听端口
        port: 9802,
        // 获取工作线程数
        workers: 12,
        // 获取临时文件夹
        temp_dir: "tmp",
        cli_colors: true,
        // 最大线程，按照引擎最大线程的一半
        max_blocking: 31
      }
    }
  ],
  // 全局配置，会被项目中的配置覆盖
  global_config: {},
  // 全局变量，允许用户提前声明一些变量，这些变量可以在所有项目的所有流中被调度和修改
  global_variable: [
    {
      key: 'name',
      // 设计器属性，引擎不会读取此属性
      name: '用户名称',
      default: 'noah',
      // 冲突防止策略
      // none: 不做任何检查，直接覆盖，在并发情况下会出现冲突
      // readonly: 只读，不允许修改
      // ol: optimistic lock，乐观锁
      // pl: pessimistic lock，悲观锁
      shared_strategy: 'readonly',
      version: 0,
      // 设计器属性，引擎不会读取此属性
      description: 'test'
    }
  ],
  // 全局环境需求
  global_requirement: [],
  // 项目配置
  module: [
    "crm-system-api"
  ],
}