/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: module
 */

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use toml;



use crate::subcommand_define;
use crate::{common::common, print_debug_msg, print_error_msg, print_info_msg};

pub fn module(sub_cmd: &subcommand_define::ModuleCmds) {
    match sub_cmd {
        subcommand_define::ModuleCmds::Create { name, no_demo } => {
            create(name, *no_demo);
        }
        subcommand_define::ModuleCmds::Build { jobs } => {
            build(jobs);
        }
        subcommand_define::ModuleCmds::List { name } => {
            print_installed_module_list(name);
        }
        subcommand_define::ModuleCmds::ListFiles { name } => {
            print_installed_module_files(name);
        }
        subcommand_define::ModuleCmds::Config { sub_cmd } => match sub_cmd {
            subcommand_define::ConfigSubCmd::Set {
                name,
                version,
                architecture,
                author,
                description,
            } => {
                set_current_module_config(name, version, architecture, author, description);
            }
            subcommand_define::ConfigSubCmd::Get {} => {
                print_current_module_config();
            }
        },
    }
}

/// 获取当前系统中已经安装的自动驾驶系统模块列表
/// # Argment
/// * 'Null'
///  # Return
///  * 'Null'
fn print_installed_module_list(name: &Option<String>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "ID",
        "Name",
        "Version",
        "Architecture",
        "Author",
        "Description",
    ]);

    match name {
        Some(name) => match common::get_installed_module_info(name) {
            Ok(module_info) => {
                table.add_row(vec![
                    "1".to_string(),
                    name.to_string(),
                    module_info.version_meta_data.version,
                    module_info.version_meta_data.platform,
                    module_info.version_meta_data.author,
                    module_info.version_meta_data.description,
                ]);

                print_info_msg!("{}", table.to_string());
            }
            Err(err) => {
                print_debug_msg!("哦豁{}", err.to_string());
            }
        },
        None => {
            let mut count: u32 = 0;
            for module in common::get_installed_module_list() {
                count += 1;
                match common::get_installed_module_info(&module) {
                    Ok(module_info) => {
                        table.add_row(vec![
                            count.to_string(),
                            module,
                            module_info.version_meta_data.version,
                            module_info.version_meta_data.platform,
                            module_info.version_meta_data.author,
                            module_info.version_meta_data.description,
                        ]);
                    }
                    Err(err) => {
                        print_error_msg!("{}", err.to_string());
                    }
                }
            }
            print_info_msg!("{}", table.to_string());
        }
    }


}

fn create(name: &str, no_demo: bool) {
    print_info_msg!("创建 {}, demo: {}", name, no_demo);
}

fn build(jobs: &u32) {
    print_info_msg!("{}", jobs);
}

/// 设置当前模块的配置信息
fn set_current_module_config(
    name: &Option<String>,
    version: &Option<String>,
    architecture: &Option<String>,
    author: &Option<String>,
    description: &Option<String>,
) {
    // 检查文件是否存在
    let mut metadata = if Path::new(common::module_constants::MODULE_META_DATA_NAME).exists() {
        // 文件存在，读取现有内容
        let mut contents = String::new();
        File::open(common::module_constants::MODULE_META_DATA_NAME)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        toml::from_str(&contents).unwrap_or_default()
    } else {
        // 文件不存在，创建新的 VersionMetaData 实例
        common::VersionMetaData::default()
    };

    // 更新元数据字段
    if let Some(name) = name {
        metadata.name = name.clone();
    }
    if let Some(version) = version {
        metadata.version = version.clone();
    }
    if let Some(architecture) = architecture {
        metadata.platform = architecture.clone();
    }
    if let Some(author) = author {
        metadata.author = author.clone();
    }
    if let Some(description) = description {
        metadata.description = description.clone();
    }

    // 序列化为 TOML 格式
    let toml_metadata = toml::to_string(&metadata).unwrap();

    // 写入文件
    let mut meta_file = File::create(common::module_constants::MODULE_META_DATA_NAME).unwrap();
    meta_file.write_all(toml_metadata.as_bytes()).unwrap();
}

/// 获取当前模块的配置信息
fn print_current_module_config() {
    if Path::new(common::module_constants::MODULE_META_DATA_NAME).exists() {
        let mut contents = String::new();
        File::open(common::module_constants::MODULE_META_DATA_NAME)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        let metadata: common::VersionMetaData = toml::from_str(&contents).unwrap();
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["模块名称", "版本号", "平台", "作者", "描述"])
            .add_row(vec![
                metadata.name,
                metadata.version,
                metadata.platform,
                metadata.author,
                metadata.description,
            ]);
        print_info_msg!("{}", table.to_string());
    } else {
        print_error_msg!(
            "包信息读取失败， 原因: {} 不存在",
            common::module_constants::MODULE_META_DATA_NAME
        );
    }
}

/// 获取指定模块的文件信息
fn print_installed_module_files(name: &str) {
    print_debug_msg!("{}", name)
}
