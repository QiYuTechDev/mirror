pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum GoLangMirror {
    /// 阿里云镜像
    Aliyun,
}

impl GoLangMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            GoLangMirror::Aliyun => "https://mirrors.aliyun.com/goproxy/",
        }
    }
}

#[derive(StructOpt, Debug)]
/// go language GOPROXY 镜像管理
///
/// 因为 go-lang 使用 GOPROXY 环境变量设置 mirror 的配置
///
/// 因此无法自动设置并且保存
///
/// 请使用 export GOPROXY=`mirror golang show $name` 来设置需要使用的镜像地址
pub enum GoLangCli {
    /// 获取当前使用的镜像
    ///
    /// 注意: 如果获取到的为空则使用系统的默认值
    Get,
    /// 获取镜像地址
    Show(GoLangMirror),
}

impl GoLangCli {
    pub fn run(&self) {
        match self {
            GoLangCli::Get => match std::env::var("GOPROXY") {
                Ok(v) => println!("{}", v),
                Err(_) => eprintln!("get `GOPROXY` env failed!"),
            },
            GoLangCli::Show(v) => println!("{}", v.get_mirror()),
        }
    }
}
