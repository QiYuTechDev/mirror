use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// ubuntu bionic 镜像
pub enum BionicMirror {
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

impl BionicMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            BionicMirror::Aliyun => include_str!("bionic/aliyun.txt"),
            BionicMirror::Netease => include_str!("bionic/netease.txt"),
            BionicMirror::Tuna => include_str!("bionic/tuna.txt"),
        }
    }
}
