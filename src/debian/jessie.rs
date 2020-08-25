use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// debian jessie 镜像
pub enum JessieMirror {
    /// 阿里云镜像
    /// doc: https://developer.aliyun.com/mirror/debian
    Aliyun,
}

impl JessieMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            JessieMirror::Aliyun => include_str!("jessie/aliyun.txt"),
        }
    }
}
