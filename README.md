# OxideDB 功能进度

> **开发状态**：进行中  
> **当前完成度**：17.39% (8/46)

---

## 基础功能（核心引擎）

- [ ] 支持 JSON 格式文档的插入与读取
- [ ] 支持自定义字段类型（string, int, float, bool, datetime 等）
- [ ] 支持创建/删除集合
- [ ] 支持集合的元数据定义与存储（manifest）
- [ ] 内置字段验证机制（类型约束）
- [ ] 基于列的存储结构（每个字段一个文件）
- [ ] 内部字段压缩与编码（未来计划）
- [ ] 写前日志（WAL）实现
- [ ] 启动时自动恢复未提交的事务
- [ ] 简单事务提交/回滚机制

---

## 查询引擎

- [ ] 支持链式查询构建器（如 `.filter().project().limit()`)
- [ ] 支持命令行查询语法（filter 表达式 + 投影字段）
- [ ] 支持简单比较运算符（=, >, <, !=, in, not in）
- [ ] 支持嵌套字段访问（如 `user.name`）
- [ ] 支持分页/跳过（skip, limit）
- [ ] 支持排序（升序/降序，单字段）

---

## 命令行接口（CLI）

- [x] `oxide init`：初始化数据库目录
- [x] `oxide create <collection>`：创建集合
- [x] `oxide insert <collection> <document>`：插入文档
- [x] `oxide query <collection> [...]`：查询文档
- [x] `oxide delete <collection> --filter`：按条件删除
- [x] `oxide list`：列出所有集合及字段定义
- [x] `oxide info <collection>`：显示集合统计信息
- [x] 支持交互式 Shell 模式
- [ ] 支持输出为 JSON / 表格格式

---

## 嵌入式 API（Rust）

- [ ] 提供 `Database` 对象，统一管理集合
- [ ] 提供 `Collection` API（insert, query, delete）
- [ ] 查询构建器支持链式调用
- [ ] 提供 `Document` 类型封装（类似 JSON value）
- [ ] 支持错误类型统一处理（Result/Errors）

---

## 网络服务

- [ ] 提供 RESTful HTTP 接口（启用后端服务）
- [ ] 提供 WebSocket 推送（订阅更新）
- [ ] 提供 `/collections`, `/insert`, `/query`, `/stats` 等 API
- [ ] 提供认证机制（token 或 basic auth）

---

## 图形界面（Tauri + React）

- [ ] 左侧导航：列出所有集合
- [ ] 集合视图：展示文档为表格
- [ ] 文档编辑器：支持 JSON 编辑与保存
- [ ] 查询面板：可视化链式查询构建
- [ ] 设置页面：修改数据库路径、导入/导出

---

## 测试与开发辅助

- [ ] 提供 `examples/` 示例代码（嵌入式 + 服务模式）
- [ ] 提供 `tests/` 单元测试与性能测试
- [ ] 提供构建脚本 `scripts/build.sh`
- [ ] 提供开发启动脚本 `scripts/dev.sh`
- [ ] Windows 支持（PowerShell/.bat 脚本）

---

