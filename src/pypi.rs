use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Pypi 镜像
/// 请确保 pip 已经安装到系统中
pub enum PypiMirror {
    /// 官方
    Official,
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/pypi
    Aliyun,
    /// 网易 163 镜像
    /// doc: http://mirrors.163.com/.help/pypi.html
    Netease,
    /// 清华(tuna)大学镜像
    /// doc: https://mirrors.tuna.tsinghua.edu.cn/help/pypi/
    Tuna,
    /// 华为云镜像
    /// 官网: https://mirrors.huaweicloud.com/
    HuaWei,
}

impl PypiMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            PypiMirror::Official => "https://pypi.org/simple",
            PypiMirror::Aliyun => "https://mirrors.aliyun.com/pypi/simple",
            PypiMirror::Netease => "https://mirrors.163.com/pypi/simple",
            PypiMirror::Tuna => "https://pypi.tuna.tsinghua.edu.cn/simple",
            PypiMirror::HuaWei => "https://repo.huaweicloud.com/python/",
        }
    }
}

#[derive(StructOpt, Debug)]
/// python pypi 全局镜像管理
///
/// 如果您使用 pipenv 或者 poetry 来管理 python 虚拟环境
/// 可以使用 `mirror pypi show name` 来获取镜像的地址
///
/// 使用命令获取镜像代理:
///     pip config get global.index-url
///
/// 使用命令设置镜像代理:
///     pip config set global.index-url ${mirror}
///
///
/// Poetry 设置: [使用 pyproject.toml 文件中]
///
/// [[tool.poetry.source]]
/// name = "xxx" # 随便起一个好记忆的名字
/// url = "xxx"  # 使用 mirror 地址
/// default = true
///
pub enum PypiCli {
    /// 获取当前使用的镜像
    ///
    /// 使用 `pip config get global.index-url` 命令获取当前的镜像地址
    Get,
    /// 显示镜像的地址
    Show(PypiMirror),
    /// 设置 pypi 镜像
    ///
    /// 使用 `pip config set global.index-url $url` 设置镜像地址
    Set(PypiMirror),
}

impl PypiCli {
    pub fn run(&self) {
        let pip = crate::helper::get_cmd_path("pip3");

        match self {
            PypiCli::Get => {
                crate::helper::run_cmd(pip.as_str(), &["config", "get", "global.index-url"])
            }
            PypiCli::Show(m) => println!("{}", m.get_mirror()),
            PypiCli::Set(m) => {
                let mirror = m.get_mirror();
                crate::helper::run_cmd(pip.as_str(), &["config", "set", "global.index-url", mirror])
            }
        }
    }
}
