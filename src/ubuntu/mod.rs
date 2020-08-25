use structopt::StructOpt;

mod bionic;
mod focal;
mod xenial;

#[derive(StructOpt, Debug)]
/// ubuntu apt 源镜像
///
/// 可以使用命令 mirror ubuntu xenial $name > /etc/apt/sources.list
/// 来覆盖已有的 apt 源
///
/// 覆盖 /etc/apt/sources.list 文件
///
pub enum UbuntuCli {
    /// xenial 16.04 lts 镜像
    Xenial(xenial::XenialMirror),
    /// bionic 18.04 lts 镜像
    Bionic(bionic::BionicMirror),
    /// focal  20.04 lts 镜像
    Focal(focal::FocalMirror),
}

impl UbuntuCli {
    pub fn run(&self) {
        let mirror = match self {
            UbuntuCli::Xenial(m) => m.get_mirror(),
            UbuntuCli::Bionic(m) => m.get_mirror(),
            UbuntuCli::Focal(m) => m.get_mirror(),
        };

        println!("{}", mirror);
    }
}
