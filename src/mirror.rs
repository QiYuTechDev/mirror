use structopt::StructOpt;

/// 镜像网站
#[derive(StructOpt, Debug)]
pub enum MirrorCli {
    /// 阿里云的镜像网站
    #[structopt(name = "aliyun")]
    AliYun,
    /// 网易镜像
    #[structopt(name = "163")]
    NetEase,
    /// 清华镜像
    #[structopt(name = "tuna")]
    Tuna,
}

impl MirrorCli {
    pub fn run(&self) {
        match self {
            MirrorCli::AliYun => println!("https://developer.aliyun.com/mirror/"),
            MirrorCli::NetEase => println!("https://mirrors.163.com/"),
            MirrorCli::Tuna => println!("https://mirrors.tuna.tsinghua.edu.cn/"),
        }
    }
}
