use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum ComposerMirror {
    /// 官方源
    Official,
    /// 阿里云 composer 镜像
    Aliyun,
}

impl ComposerMirror {
    pub fn get_mirror(&self) -> &'static str {
        match self {
            ComposerMirror::Official => "https://repo.packagist.org",
            ComposerMirror::Aliyun => "https://mirrors.aliyun.com/composer/",
        }
    }
}

#[derive(StructOpt, Debug)]
/// php composer 全局镜像管理
///
/// 如果需要设置 单个项目 的镜像地址
/// 可以使用命令:
///
/// composer config repo.packagist composer `mirror composer show name`
///
/// 或者自己手工编辑 composer.json 文件
///
/// 使用命令设置镜像代理:
///     composer config -g repo.packagist composer ${mirror}
pub enum ComposerCli {
    /// 设置全局镜像
    /// 使用 `composer config -g repo.packagist composer $url` 设置镜像地址
    Set(ComposerMirror),
    /// 展示镜像地址
    Show(ComposerMirror),
}

impl ComposerCli {
    pub fn run(&self) {
        let composer = crate::helper::get_cmd_path("composer");

        match self {
            ComposerCli::Set(m) => {
                let m = m.get_mirror();
                crate::helper::run_cmd(
                    composer.as_str(),
                    &["config", "-g", "repo.packagist", "composer", m],
                );
            }
            ComposerCli::Show(m) => {
                println!("{}", m.get_mirror());
            }
        }
    }
}
