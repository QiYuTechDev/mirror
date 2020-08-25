use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// ubuntu xenial 镜像
pub enum XenialMirror {
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/ubuntu
    Aliyun,
    /// 网易 163 镜像
    /// doc: http://mirrors.163.com/.help/ubuntu.html
    Netease,
    /// 清华大学(tuna)镜像
    /// doc: https://mirrors.tuna.tsinghua.edu.cn/help/ubuntu/
    Tuna,
}

impl XenialMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            XenialMirror::Aliyun => include_str!("xenial/aliyun.txt"),
            XenialMirror::Netease => include_str!("xenial/netease.txt"),
            XenialMirror::Tuna => include_str!("xenial/tuna.txt"),
        }
    }
}
