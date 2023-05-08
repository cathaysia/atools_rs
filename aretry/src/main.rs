use std::process::Command;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// 重试的次数。负数表示无限尝试
    #[arg(short, long, default_value_t = 5)]
    count: i32,
    cmd: Vec<String>,
}

fn main() {
    let args = Args::parse();
    if args.cmd.len() == 0 {
        return;
    }

    let scmd = &args.cmd[0];
    let sargs = &args.cmd[1..].to_vec();

    for _ in 0..args.count {
        let ret = Command::new(scmd).args(sargs).spawn();
        if let Ok(mut v) = ret {
            let ret = v.wait();
            if let Ok(_) = ret {
                return;
            } else {
                continue;
            }
        } else {
            println!("{:?}", ret);
        }
    }
}
