use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// debian buster 镜像
pub enum BusterMirror {
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/debian
    Aliyun,
}

impl BusterMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            BusterMirror::Aliyun => include_str!("buster/aliyun.txt"),
        }
    }
}
