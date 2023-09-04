use regex::Regex;
use std::env;
use std::process;
use colored::Colorize;
mod search_file;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("使用方式: {} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式 '{}': {}", args[2], err);
            process::exit(1);
        }
    };

    match search_file::find(&args[1], &regex) {
        Ok((matches, cnt)) => {
            if matches.is_empty() {
                println!("未找到匹配项");
            } else {
                println!("找到以下匹配项");               
                if args.len() <= 3 || (args[3] != "-v" && args[3] != "--verbose") { 
                    let mut i = 0;
                    for file in &matches {
                        println!("{}", file.green().bold());
                        i += 1;
                        if i >= cnt {
                            break;
                        }
                    }                  
                } else {
                    let mut i = 0;
                    for file in &matches {
                        if i < cnt {
                            println!("{}", file.green().bold());
                        } else {
                            println!("{}", file.red());
                        }
                        i += 1;
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("发生错误:{}", error);
            process::exit(1);
        }
    }
}