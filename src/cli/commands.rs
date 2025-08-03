//命令各键值
#[derive(Debug)]
pub enum Command {
    Create { path: String },
    Open { path: Option<String> },
    Insert { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    List,
    Clear,
    Info,
    Serve { port: u16 },
}
//传参结构体
#[derive(Debug)]
pub struct ParsedArgs {
    pub command: Command,
}
