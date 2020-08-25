use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum FocalMirror {
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/ubuntu
    Aliyun,
}

impl FocalMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            FocalMirror::Aliyun => include_str!("focal/aliyun.txt"),
        }
    }
}
