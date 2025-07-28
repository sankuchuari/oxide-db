// 对外暴露的 API
pub mod api;

// 内部核心引擎
pub mod core;

// 命令行支持（如果你想让 CLI 工具也能通过库调用）
pub mod cli;

// 网络接口（可选）
pub mod net;

// 工具函数
pub mod util;
