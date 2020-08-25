use structopt::StructOpt;

mod buster;
mod jessie;
mod stretch;
mod wheezy;

#[derive(StructOpt, Debug)]
/// debian apt 源镜像
///
/// 可以使用命令 mirror debian
/// 来覆盖已有的 apt 源
///
/// 覆盖 /etc/apt/sources.list 文件
///
pub enum DebianCli {
    /// debian 7.x (wheezy)
    Wheezy(wheezy::WheezyMirror),
    /// debian 8.x (jessie)
    Jessie(jessie::JessieMirror),
    /// debian 9.x (stretch)
    Stretch(stretch::StretchMirror),
    /// debian 10.x (buster)
    Buster(buster::BusterMirror),
}

impl DebianCli {
    pub fn run(&self) {
        let mirror = match self {
            DebianCli::Wheezy(m) => m.get_mirror(),
            DebianCli::Jessie(m) => m.get_mirror(),
            DebianCli::Stretch(m) => m.get_mirror(),
            DebianCli::Buster(m) => m.get_mirror(),
        };

        println!("{}", mirror);
    }
}
