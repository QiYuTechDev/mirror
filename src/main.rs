use structopt::StructOpt;

mod composer;
mod debian;
mod docker;
mod gem;
mod golang;
mod helper;
mod mirror;
mod npm;
mod pypi;
mod ubuntu;
mod yarn;

#[derive(StructOpt, Debug)]
#[structopt(name = "mirror")]
/// 镜像管理/配置工具
enum CliOpt {
    Composer(composer::ComposerCli),
    Docker(docker::DockerCli),
    Debian(debian::DebianCli),
    Gem(gem::GemCli),
    Golang(golang::GoLangCli),
    Npm(npm::NpmCli),
    Pypi(pypi::PypiCli),
    Ubuntu(ubuntu::UbuntuCli),
    Yarn(yarn::YarnCli),
    Mirror(mirror::MirrorCli),
}

fn main() {
    let opt: CliOpt = CliOpt::from_args();
    match opt {
        CliOpt::Composer(c) => c.run(),
        CliOpt::Docker(c) => c.run(),
        CliOpt::Debian(c) => c.run(),
        CliOpt::Gem(c) => c.run(),
        CliOpt::Golang(c) => c.run(),
        CliOpt::Npm(c) => c.run(),
        CliOpt::Pypi(c) => c.run(),
        CliOpt::Ubuntu(c) => c.run(),
        CliOpt::Yarn(c) => c.run(),
        CliOpt::Mirror(c) => c.run(),
    }
}
