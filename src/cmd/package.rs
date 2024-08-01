/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-31
 * File: package
 */

use crate::subcommand_define;
use crate::subcommand_define::PackageCmds;

pub fn package(sub_cmd: &subcommand_define::PackageCmds) {
    match sub_cmd {
        subcommand_define::PackageCmds::List {} => { print_package_list(); }
        subcommand_define::PackageCmds::Install {name, version} => { install_package(name, version); }
        subcommand_define::PackageCmds::Upgrade {name, all} => { upgrade_package(name, all); }
        subcommand_define::PackageCmds::Uninstall {name, all} => { uninstall_package(name, all); }
        subcommand_define::PackageCmds::Info {name, all} => { print_package_info(name, all); }
    }
}


/// 打印当前系统已安装的自动驾驶软件包列表
fn print_package_list() {

}

/// 安装指定的自动驾驶软件包
fn install_package(name: &str, version: &Option<String>) {

}

/// 升级指定的自动驾驶软件包
fn upgrade_package(name: &Option<String>, all: &Option<bool>) {

}


/// 卸载指定的自动驾驶软件包
fn uninstall_package(name: &Option<String>, all: &Option<bool>) {

}

/// 打印指定软件包的详细信息
fn print_package_info(name: &Option<String>, all: &Option<bool>) {

}