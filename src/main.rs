use std::env;
use OxideDB::cli::shell;

fn main() {
    //读入命令行
    let args: Vec<String> = env::args().collect();

    // args[0] 是程序本身的路径，跳过
    if args.len() < 2 {
        eprintln!("❌ 请提供一个命令。例如：oxide_db insert key value");
        return;
    }

    match shell::parse_args() {
        Ok(parsed) => {
            println!("✅ 解析成功: {:?}", parsed);
            shell::execute(parsed);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
    
}
