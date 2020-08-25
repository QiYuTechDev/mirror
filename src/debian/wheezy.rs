use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// debian wheezy 镜像
pub enum WheezyMirror {
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/debian
    Aliyun,
}

impl WheezyMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            WheezyMirror::Aliyun => include_str!("wheezy/aliyun.txt"),
        }
    }
}
