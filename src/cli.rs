/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: cli
 */

use clap::{ColorChoice, CommandFactory, Parser};
use crate::commands;
use crate::subcommand_define;

/// 创建 cli 结构体， 用于解析命令行参数
#[derive(Parser)]
#[command(
    name = "waytous",
    version = "0.1.0",
    author = "daojin.xu101@gmail.com",
    about = "waytous",
)]
pub struct Cli {
    /// 创建 cmds 字段，用于存储解析的子命令
    #[command(subcommand)]
    pub cmds: Option<MainCmds>,

}

/// 枚举定义主命令下的所有子命令
#[derive(Parser)]
pub enum MainCmds {
    #[command(about = "软件包的创建、编译、构建、打包等操作", color = ColorChoice::Never)]
    Module {
        /// 创建 sub_cmd 字段，用于存储解析的子命令
        #[command(subcommand)]
        sub_cmd: subcommand_define::ModuleCmds,
    },

    #[command(about = "OTA 制品的创建、读取、安装、拷贝、同步、修改等操作")]
    Artifact {
        /// 创建 sub_cmd 字段，用于存储 artifact 解析器的子命令
        #[command(subcommand)]
        sub_cmd: subcommand_define::ArtifactCmds,
    },

    #[command(about = "生成并配置 cli 的自动补全脚本")]
    Autocompletion {},
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn run(&self) {
        match &self.cmds {
            Some(MainCmds::Module { sub_cmd }) => { commands::module::module(sub_cmd); }
            Some(MainCmds::Artifact { sub_cmd }) => { commands::artifact::artifact(sub_cmd); }
            Some(MainCmds::Autocompletion {}) => { commands::autocompletion::GenerateAutoCompletion::process(); }
            None => {
                println!("{}", Cli::command().render_help())
            }
        }
    }
}