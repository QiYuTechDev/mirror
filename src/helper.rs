use std::ffi::OsStr;

/// 运行指定的命令
pub fn run_cmd<S, I>(cmd: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = std::process::Command::new(cmd);
    command.args(args);
    let s = command.output().unwrap();
    if s.status.success() {
        let stdout = String::from_utf8(s.stdout).unwrap();
        print!("{}", stdout)
    } else {
        let stderr = String::from_utf8(s.stderr).unwrap();
        print!("{}", stderr)
    }
}

/// 查看 `命令` 的绝对路径
pub fn get_cmd_path(cmd: &str) -> String {
    match which::which(cmd) {
        Ok(o) => o.to_str().unwrap().to_owned(),
        Err(e) => {
            eprintln!("无法找到 {} 命令! 错误: {}", cmd, e);
            std::process::exit(1);
        }
    }
}
