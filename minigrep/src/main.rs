use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("解析参数时遇到问题：{err}");
        process::exit(1);
    });

    println!("在文件 {} 中检索：{}", config.file_path, config.query);
    if let Err(e) = run(config) {
        println!("应用程序错误：{e}");
        process::exit(1);
    }
}

