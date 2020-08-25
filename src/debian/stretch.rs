use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// debian stretch 镜像
pub enum StretchMirror {
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/debian
    Aliyun,
}

impl StretchMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            StretchMirror::Aliyun => include_str!("stretch/aliyun.txt"),
        }
    }
}
