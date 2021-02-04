use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum GemMirror {
    /// 官方
    Official,
    /// 阿里云镜像
    Aliyun,
    /// 清华大学(tuna)镜像
    /// doc: https://mirrors.tuna.tsinghua.edu.cn/help/rubygems/
    Tuna,
    /// 华为云镜像
    /// 官网: https://mirrors.huaweicloud.com/
    HuaWei,
}

impl GemMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            GemMirror::Official => "https://rubygems.org/",
            GemMirror::Aliyun => "https://mirrors.aliyun.com/rubygems/",
            GemMirror::Tuna => "https://mirrors.tuna.tsinghua.edu.cn/rubygems/",
            GemMirror::HuaWei => "https://repo.huaweicloud.com/ruby/ruby/",
        }
    }
}

#[derive(StructOpt, Debug)]
/// ruby gem 镜像管理
/// 请确保 gem 已经安装到系统中
///
/// 使用命令获取镜像代理:
///     gem sources
///
/// 使用命令设置镜像代理:
///     gem sources -a ${mirror}
pub enum GemCli {
    /// 获取当前的 ruby gem 源
    Get,
    /// 添加一个 gem 镜像
    /// 使用 `gem sources -a $url` 命令添加
    Add(GemMirror),
    /// 删除已经存在一个 gem 镜像
    /// 使用 `gem sources --remove $url` 命令删除
    Del(GemMirror),
    /// 展示镜像的 URL
    Show(GemMirror),
}

impl GemCli {
    pub fn run(&self) {
        let gem = crate::helper::get_cmd_path("gem");

        match self {
            GemCli::Get => {
                crate::helper::run_cmd(gem.as_str(), &["sources"]);
            }
            GemCli::Add(mirror) => {
                let m = mirror.get_mirror();
                crate::helper::run_cmd(gem.as_str(), &["sources", "-a", m])
            }
            GemCli::Del(mirror) => {
                let m = mirror.get_mirror();
                crate::helper::run_cmd(gem.as_str(), &["sources", "--remove", m]);
            }
            GemCli::Show(m) => {
                println!("{}", m.get_mirror());
            }
        }
    }
}
