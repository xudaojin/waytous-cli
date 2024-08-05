/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: subcommand_define
 */

use clap::Parser;

/// 枚举定义 module 命令下的所有子命令
#[derive(Parser)]
pub enum ModuleCmds {
    #[command(about = "创建一个模块的目录结构")]
    Create {
        #[arg(long, required = true, help = "模块名称")]
        name: String,

        #[arg(long, default_value_t = false, help = "是否把 demo 工程拷贝到当前工程中, 默认不拷贝")]
        no_demo: bool,
    },

    #[command(about = "构建当前模块")]
    Build {
        #[arg(long, help = "构建时所需的线程数量")]
        jobs: u32,
    },

    #[command(about = "设置或查看当前模块的配置信息")]
    Config {
        #[command(subcommand)]
        sub_cmd: ConfigSubCmd,
    },

    #[command(about = "获取指定模块的信息[名称、版本号、平台、作者、描述] 等")]
    Get {
        #[command(subcommand)]
        sub_cmd: GetSubCmd,
    },

    #[command(about = "列出当前系统中已经安装的所有模块")]
    List {},
}

/// 枚举定义 module config 命令下的所有子命令
#[derive(Parser)]
pub enum ConfigSubCmd {

    #[command(about = "设置当前模块的配置信息")]
    Set {
        #[arg(long, help = "模块名称")]
        name: Option<String>,

        #[arg(long, help = "模块版本号")]
        version: Option<String>,

        #[arg(long, help = "模块所属平台")]
        platform: Option<String>,

        #[arg(long, help = "模块作者信息")]
        author: Option<String>,

        #[arg(long, help = "模块描述信息")]
        description: Option<String>,
    },

    #[command(about = "获取当前模块的配置信息")]
    Get {

    },
}


/// 枚举定义 waytous module get 命令下的所有子命令
#[derive(Parser)]
pub enum GetSubCmd {
    #[command(about = "获取当前模块的详细信息")]
    Info {},
}


/// 枚举定义 waytous artifact 命令下的所有子命令
#[derive(Parser)]
pub enum ArtifactCmds {
    #[command(about = "创建一个用于 OTA 升级的制品")]
    Write {
        #[command(subcommand)]
        sub_cmd: WriteSubCmd,
    },

    #[command(about = "读取指定 OTA 制品的信息")]
    Read {
        #[arg(short, long, help = "需要读取的 OTA 制品名称")]
        file: String,
    },

    #[command(about = "在当前环境下安装指定的 OTA 制品")]
    Install {},

    #[command(about = "修改指定的 OTA 制品")]
    Modify {},

    #[command(about = "拷贝指定的 OTA 制品到知道你的位置")]
    Cp {},

    #[command(about = "列出指定 OTA 制品仓库中的所有制品")]
    List {},

    #[command(about = "同步 OTA 制品到本地或云端制品仓库")]
    Rsync {},
}


/// 枚举定义 waytous artifact write 命令下的所有子命令
#[derive(Parser)]
pub enum WriteSubCmd {
    #[command(about = "创建一个模块类型的 OTA 制品")]
    ModuleImage {
        #[arg(short = 'T', long, value_parser = ["deb", "run"], help = "指定模块类型")]
        type_value: String,

        #[arg(short = 'n', long, help = "制品名称, 格式: ht-truck")]
        artifact_name: String,

        #[arg(long, help = "软件版本号, ex: 1.0.0")]
        software_version: String,

        #[arg(long, value_parser = ["release", "debug"], default_value_t = String::from("release"),  help = "制品类型")]
        mode: String,

        #[arg(short, long, num_args = 1.., value_delimiter = ' ', help = "打包的文件")]
        files: Vec<String>,
    },

    #[command(about = "创建一个带有启动引导项的 image OTA 制品")]
    Boostrap {},

    #[command(about = "创建一个带有 rootfs 磁盘映像的 image OTA 制品")]
    RootfsImage {},
}


/// 枚举定义 waytous deploy 命令下的所有子命令
#[derive(Parser)]
pub enum DeployCmds {
    #[command(about = "在当前系统部署自动驾驶所需的基础依赖环境")]
    Deps {},
}

/// 枚举定义 waytous init 命令下的所有子命令
#[derive(Parser)]
pub enum InitCmds {
    #[command(about = "初始化当前系统的一些硬件接口， 如[CAN NET Serial] 等")]
    Interface {
        #[command(subcommand)]
        sub_cmd: InterfaceSubCmd,
    },

    #[command(about = "初始化当前系统的一些公共配置")]
    Config {
        #[command(subcommand)]
        sub_cmd: ConfigSubCmd,
    },
}

/// 枚举定义 waytous init interface 命令下的所有子命令
#[derive(Parser)]
pub enum InterfaceSubCmd {
    #[command(about = "初始化指定的 CAN 接口")]
    Can {},

    #[command(about = "初始化指定的网络接口")]
    Net {},

    #[command(about = "初始化指定的串口接口")]
    Serial {},
}
