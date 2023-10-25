use std::process::Command;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// 重试的次数。负数表示无限尝试
    #[arg(short, long, default_value_t = 5)]
    count: i32,
    cmd: Vec<String>,
    #[arg(long, default_value_t = false)]
    always: bool,
}

fn main() {
    let args = Args::parse();
    if args.cmd.is_empty() {
        return;
    }

    let scmd = &args.cmd[0];
    let sargs = &args.cmd[1..].to_vec();

    if args.count < 0 {
        loop {
            let ret = Command::new(scmd).args(sargs).spawn();
            if let Ok(mut v) = ret {
                let ret = v.wait();
                if let Ok(retval) = ret {
                    if !retval.success() {
                        continue;
                    }
                    if !args.always {
                        return;
                    }
                } else {
                    continue;
                }
            } else {
                println!("{:?}", ret);
            }
        }
    } else {
        for _ in 0..args.count {
            let ret = Command::new(scmd).args(sargs).spawn();
            if let Ok(mut v) = ret {
                let ret = v.wait();
                if let Ok(retval) = ret {
                    if !retval.success() {
                        continue;
                    }
                    if !args.always {
                        return;
                    }
                } else {
                    continue;
                }
            } else {
                println!("{:?}", ret);
            }
        }
    }
}
