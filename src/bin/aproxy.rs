use log::debug;
use std::{env, process::Command, vec};

fn entry_vars(name: &str, value: &str) {
    if env::var(name).is_ok() {
        return;
    }

    env::set_var(name, value);
}

fn main() {
    env_logger::init();

    let argv: Vec<_> = std::env::args().collect();
    if argv.len() <= 1 {
        return;
    }

    let aproxy = env::var("APROXY");
    if aproxy.is_err() {
        Command::new(&argv[1]).args(&argv[1..]).spawn().unwrap();
        return;
    }

    let aproxy = aproxy.unwrap();
    if aproxy.is_empty() {
        Command::new(&argv[1]).args(&argv[1..]).spawn().unwrap();
        return;
    }

    let aproxy_http = env::var("APROXY_HTTP").unwrap_or(aproxy.clone());
    let aproxy_https = env::var("APROXY_HTTPS").unwrap_or(aproxy.clone());
    let aproxy_ftp = env::var("APROXY_FTP").unwrap_or(aproxy.clone());

    // 设置环境变量
    entry_vars("ALL_PROXY", &aproxy);
    entry_vars("http_proxy", &aproxy_http);
    entry_vars("https_proxy", &aproxy_https);
    entry_vars("ftp_proxy", &aproxy_ftp);
    entry_vars("GOPROXY", "https://goproxy.cn");
    entry_vars("PUB_HOSTED_URL", "https://pub.flutter-io.cn");
    entry_vars("FLUTTER_STORAGE_BASE_URL", "https://storage.flutter-io.cn");

    let cmd = &argv[1];
    let mut args = Vec::new();
    // 添加额外参数
    match cmd.as_str() {
        "curl" => {
            let extra_args = vec!["--proxy".to_string(), aproxy.clone()];
            args.extend(extra_args);
        }
        "git" => {
            let extra_args = vec![
                "-c".to_string(),
                format!("http.proxy={}", aproxy),
                "-c".to_string(),
                format!("https.proxy={}", aproxy),
                "-c".to_string(),
                "http.sslVerify=false".to_string(),
                "-c".to_string(),
                "https.sslVerify=false".to_string(),
            ];
            args.extend(extra_args);
        }
        "svn" => {
            let pos = aproxy.find(":").unwrap();
            let extra_args = vec![
                "--config-option".to_string(),
                format!("servers:global:http-proxy-host={}", &aproxy[..pos]),
                "--config-option".to_string(),
                format!("servers:global:http-proxy-port={}", &aproxy[pos..]),
            ];
            args.extend(extra_args);
        }
        &_ => {}
    }

    args.extend(argv[2..].to_vec());
    debug!("cmd = {}, args = {:?}", cmd, args);

    let _ = Command::new(&cmd).args(&args).spawn().unwrap().wait();
}
