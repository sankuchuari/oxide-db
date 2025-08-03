use std::env;
use crate::cli::commands::{Command, ParsedArgs};

//解析cli命令
pub fn parse_args() -> Result<ParsedArgs, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("❌ 缺少命令参数".into());
    }

    let cmd = args[0].as_str();
    let rest = &args[1..];

    let command = match cmd {
        "create" if rest.len() == 1 => Command::Create { path: rest[0].clone() },
        "open" => Command::Open {
            path: rest.get(0).cloned(),
        },
        "insert" if rest.len() == 2 => Command::Insert {
            key: rest[0].clone(),
            value: rest[1].clone(),
        },
        "get" if rest.len() == 1 => Command::Get {
            key: rest[0].clone(),
        },
        "delete" if rest.len() == 1 => Command::Delete {
            key: rest[0].clone(),
        },
        "list" if rest.is_empty() => Command::List,
        "clear" if rest.is_empty() => Command::Clear,
        "info" if rest.is_empty() => Command::Info,
        "serve" => {
            let port = if let Some(i) = rest.iter().position(|s| s == "--port") {
                rest.get(i + 1)
                    .and_then(|p| p.parse::<u16>().ok())
                    .unwrap_or(8080)
            } else {
                8080
            };
            Command::Serve { port }
        }
        _ => return Err(format!("❌ 无效命令或参数: {:?}", args)),
    };

    Ok(ParsedArgs { command })
}

//调用具体函数
pub fn execute(args: ParsedArgs) {
    match args.command {
        Command::Create { path } => println!("🧱 创建数据库: {}", path),
        Command::Open { path } => println!("📂 打开数据库: {:?}", path),
        Command::Insert { key, value } => println!("🧪 插入: {} = {}", key, value),
        Command::Get { key } => println!("🔍 获取: {}", key),
        Command::Delete { key } => println!("❌ 删除: {}", key),
        Command::List => println!("🧾 列出所有键"),
        Command::Clear => println!("🧹 清空数据库"),
        Command::Info => println!("📋 数据库信息"),
        Command::Serve { port } => println!("🌐 启动 Web 服务: http://localhost:{}", port),
    }
}
