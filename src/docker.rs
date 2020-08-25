use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum DockerMirror {
    /// microsfot Azure
    Azure,
    /// docker 中国区镜像
    DockerChina,
    /// 网易镜像
    Netease,
    /// 中国科技大学
    Ustc,
}

impl DockerMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            DockerMirror::Azure => "https://dockerhub.azk8s.cn",
            DockerMirror::DockerChina => "https://registry.docker-cn.com",
            DockerMirror::Netease => "http://hub-mirror.c.163.com",
            DockerMirror::Ustc => "https://docker.mirrors.ustc.edu.cn",
        }
    }
}

#[derive(StructOpt, Debug)]
/// docker 镜像源
///
/// 在 linux 上 请修改: /etc/docker/daemon.json
///
/// 添加:
/// {
///  "registry-mirrors": ["list", "of", "mirrors"]
/// }
pub enum DockerCli {
    /// 展示镜像地址
    Show(DockerMirror),
}

impl DockerCli {
    pub fn run(&self) {
        let mirror = match self {
            DockerCli::Show(c) => c.get_mirror(),
        };

        println!("{}", mirror);
    }
}
