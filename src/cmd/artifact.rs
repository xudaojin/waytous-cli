/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-31
 * File: artifact
 */

use crate::subcommand_define;
use colored::Colorize;
use comfy_table::presets::{UTF8_FULL, UTF8_NO_BORDERS};
use comfy_table::{ContentArrangement, Table};
use crate::common;
pub fn artifact(sub_cmd: &subcommand_define::ArtifactCmds) {
    match sub_cmd {
        subcommand_define::ArtifactCmds::Write { sub_cmd } => {
            match sub_cmd {
                subcommand_define::WriteSubCmd::ModuleImage { type_value, artifact_name, device_type, software_version, files } => {
                    write_module_image(type_value, artifact_name, device_type, software_version, files);
                }
                _ => {}
            }
        }
        subcommand_define::ArtifactCmds::Read { file } => { get_artifact_info(file); }
        subcommand_define::ArtifactCmds::Install { .. } => {}
        subcommand_define::ArtifactCmds::Modify { .. } => {}
        subcommand_define::ArtifactCmds::Cp { .. } => {}
        subcommand_define::ArtifactCmds::List { .. } => {}
        subcommand_define::ArtifactCmds::Rsync { .. } => {}
    }


}


/// 获取指定制品文件的信息
/// # Arguments
/// * file - 制品文件名称
/// # Return
/// Null
///
fn get_artifact_info(file: &str) {
    common::common::system("mender-artifact", vec!["read", file]);
    println!("{}", common::common::get_system_info().hostname);

}

/// 根据传入的参数来制作对应的 OTA 软件制品
/// # Arguments
/// * type_value - 制品类型(deb zip run tar)
/// * artifact_name - 制品名称
/// * device_type - 设备类型
/// * software_version - 软件版本
/// * files - 需要打包的文件列表
/// # Return
/// Null
fn write_module_image(type_value: &str, artifact_name: &str, device_type: &str, software_version: &str, files: &Vec<String>) {

    /// 组装最终的制品文件名称
    let artifact_full_name = format!("{}.mender", artifact_name);

    /// 组装打包所需的参数
    let mut command_args = vec![
        "write", "module-image",
        "-T", type_value,
        "-n", artifact_name,
        "-o", &artifact_full_name,
        "--device-type", device_type,
        "--software-version", software_version,
    ];

    /// 添加需要打包的文件， 并在前面带上 -f， 然后插入到 command_args 中构成完整的参数组
    for file in files {
        command_args.push("-f");
        command_args.push(file);
    }

    /// 执行制品打包命令
    if common::common::system("mender-artifact", command_args) {
        println!("ddd");
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_NO_BORDERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["制品类型", "制品文件名称", "适用平台", "版本信息", "文件"])
            .add_row(vec![
                type_value,
                &artifact_full_name,
                device_type,
                software_version,
                &files.join("\n"),
            ]);
        println!("{}", table.to_string().blue());
    }else {
        println!("{}", "Error".red());
    }

}