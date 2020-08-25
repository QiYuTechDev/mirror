use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// NPM 镜像
pub enum NpmMirror {
    /// 官方
    Official,
    /// 淘宝镜像
    Taobao,
}

impl NpmMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            NpmMirror::Official => "https://registry.npmjs.org/",
            NpmMirror::Taobao => "https://registry.npm.taobao.org/",
        }
    }
}

#[derive(StructOpt, Debug)]
/// npm 镜像管理
/// 请确保 npm 已经安装到系统中
///
/// 使用命令获取镜像代理:
///     npm config get registry
///
/// 使用命令设置镜像代理:
///     npm config set registry ${mirror}
pub enum NpmCli {
    /// 查看当前使用的镜像
    /// 使用 `npm config get registry` 命令获取当前的镜像地址
    Get,
    /// 展示镜像地址
    Show(NpmMirror),
    /// 设置 NPM 镜像
    /// 使用 `npm config set registry $url` 设置镜像地址
    Set(NpmMirror),
}

impl NpmCli {
    /// 执行命令
    pub fn run(&self) {
        let npm = crate::helper::get_cmd_path("npm");

        match self {
            NpmCli::Get => crate::helper::run_cmd(npm.as_str(), &["config", "get", "registry"]),
            NpmCli::Show(m) => println!("{}", m.get_mirror()),
            NpmCli::Set(m) => {
                let mirror = m.get_mirror();
                crate::helper::run_cmd(npm.as_str(), &["config", "set", "registry", mirror])
            }
        }
    }
}
