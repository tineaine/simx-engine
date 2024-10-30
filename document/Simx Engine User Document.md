# Simx 用户使用手册

## 基础概念

### Simx引擎

#### 介绍

Simx引擎此处特指 Rust 开发的，最小的Simx蓝图运行器，通过此软件运行Simx项目或Flow文件。

#### 起源

Simx Engine的前身是通过 Go 开发的Similar Flow Engine（SFE，2020）流引擎，目前已经不再维护，更早的起源则是大学时的毕业设计，使用C#
开发的Server Manager项目。

后来此项目增加了RPA部分的功能，但由于希望兼容Linux环境，因此使用Go进行了第一次重写，在2021年，采用Rust进行第二次重写，命名为Simx。

#### Simx流引擎和Glue平台的区别

Glue 包含流引擎（Simx Engine）、设计单元（Design Unit）、管理单元（Management Unit）、流编译器（Flow
Compiler），是一套完整的解决方案。目前仍然处于早期开发阶段。

### 工作空间

工作空间是多个项目的集合，项目必然存在于一个工作空间中，如果未自行创建，默认为 default 工作空间。

### 项目

Simx 项目是指多个蓝图，脚本，插件，项目配置组成的业务集合，与其他编程语言中项目的概念类似。

### 蓝图

可被Simx 引擎解析并执行的文件，以flow/sf为扩展名。

- flow：默认蓝图，可以直接通过编辑器打开，未进行加密和混淆
- sf：加密和混淆后的蓝图，需要通过解析才能使用

蓝图基于Json文件，具有严格的格式规范，一般情况下不适合手动编辑，通常由设计器生成。

### 工作空间

工作空间是多个项目的集合，大型业务可能具有上千个流，数十个项目，以方便开发和管理，因此可以通过工作空间同时对多个项目进行管理和控制。

工作空间可以是一个或多个，默认创建的项目均在 default 工作空间下。

### 蓝图

蓝图中组织节点的数据结构，其中包含了流中用到的所有流节点。

### 流节点

也称为执行单元，可以包含运行处理器的信息。

流节点具有处理器使用的所有属性，这部分内容正常应该由设计器生成。

### 流处理器

对应一个操作（方法），节点根据handler字符串寻找对应的处理器.

simx引擎内置处理器均以 `simx`  开头，`simx 可拆卸处理器`均以 `cn.tineaine.xxx` 开头， 其他任何`处理器`必须以`域名`为前缀，例如：
`com.tineaine.files.find`

### 引擎扩展

### 处理器扩展

### 服务扩展

### 三方扩展

```json

{
  "name": "while 循环",
  "handler": "",
  "downstream": [
    "6"
  ],
  "redress_stream": [],
  "tags": [
    "Route",
    "Loop"
  ],
  "attr": {
    "expression": "1==1",
    "interval": 0.2,
    "parallel_endpoints": true,
    "parallel_routes": false,
    "maximum_repetition": 50,
    "endpoints": [
      "5",
      "6"
    ]
  }
}

```