use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum YarnMirror {
    /// 使用官方镜像
    Official,
    /// 使用淘宝的镜像
    Taobao,
}

impl YarnMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            YarnMirror::Official => "https://registry.yarnpkg.com",
            YarnMirror::Taobao => "https://registry.npm.taobao.org",
        }
    }
}

#[derive(StructOpt, Debug)]
/// yarn 镜像设置
///
/// 使用命令获取镜像代理:
///     yarn config get registry
///
/// 使用命令设置镜像代理:
///     yarn config set registry ${mirror}
pub enum YarnCli {
    /// 查看当前使用的镜像
    /// 使用 `yarn config get registry` 命令获取当前的镜像地址
    Get,
    /// 展示镜像地址
    Show(YarnMirror),
    /// 设置 yarn 镜像
    /// 使用 `yarn config set registry $url` 设置系统镜像
    Set(YarnMirror),
}

impl YarnCli {
    pub fn run(&self) {
        let yarn = crate::helper::get_cmd_path("yarn");

        match self {
            YarnCli::Get => crate::helper::run_cmd(yarn.as_str(), &["config", "get", "registry"]),
            YarnCli::Show(m) => println!("{}", m.get_mirror()),
            YarnCli::Set(c) => {
                let m = c.get_mirror();
                crate::helper::run_cmd(yarn.as_str(), &["config", "set", "registry", m])
            }
        }
    }
}
