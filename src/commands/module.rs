/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: module
 */

use comfy_table::{ContentArrangement, Table};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use rusqlite::{Connection, params, Result};

use crate::{common, print_error_format, print_info_format};
use crate::subcommand_define;

pub fn module(sub_cmd: &subcommand_define::ModuleCmds) {
    match sub_cmd {
        subcommand_define::ModuleCmds::Create { name, no_demo } => { create(name, *no_demo); }
        subcommand_define::ModuleCmds::Build { jobs } => { build(jobs); }
        subcommand_define::ModuleCmds::Set { name, version, platform, author, description } => { set_config(name, version, platform, author, description).expect("TODO: panic message"); }
        subcommand_define::ModuleCmds::List { .. } => { print_installed_module_list(); }
        subcommand_define::ModuleCmds::Get { sub_cmd } => {
            match sub_cmd {
                subcommand_define::GetSubCmd::Info { .. } => {
                    get_config_info();
                }
            }
        }
    }
}


/// 获取当前系统中已经安装的自动驾驶系统模块列表
/// # Argment
/// * 'Null'
///  # Return
///  * 'Null'
fn print_installed_module_list() {
    print_info_format!("TODO: print installed module list");
}

fn create(name: &str, no_demo: bool) {
    print_info_format!("创建 {}, demo: {}", name, no_demo);
}

fn build(jobs: &u32) {
    print_info_format!("{}", jobs);
}


fn set_config(name: &Option<String>, version: &Option<String>, platform: &Option<String>, author: &Option<String>, description: &Option<String>) -> Result<()> {
    let mut conn = Connection::open(common::common::MODULE_META_DATA_DB)?;

    // 创建表，如果表不存在
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meta (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE,
            version TEXT,
            platform TEXT,
            author TEXT,
            description TEXT,
            UNIQUE(name)
        )",
        [],
    )?;

    // 检查 meta 是否为空, 如果为空， 则对其初始化
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM meta", [], |r| r.get(0))?;
    let meta_fields = vec![
        ("name", name),
        ("version", version),
        ("platform", platform),
        ("author", author),
        ("description", description),
    ];

    let transaction = conn.transaction()?;

    if count == 0 {
        // 插入新的记录
        for (field, value) in &meta_fields {
            if let Some(value) = value {
                transaction.execute(
                    &format!("INSERT INTO meta ({}) VALUES (?1)", field),
                    params![value],
                )?;
            }
        }
    } else {
        // 更新现有记录
        for (field, value) in &meta_fields {
            if let Some(value) = value {
                transaction.execute(
                    &format!("UPDATE meta SET {} = ?1 WHERE id = 1", field),
                    params![value],
                )?;
            }
        }
    }

    transaction.commit()?;
    Ok(())
}

fn get_config_info() {

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["模块名称", "版本号", "适用平台", "作者", "描述"]);
    match common::common::get_module_meta() {
        Ok(meta) => {
            table.add_row(vec![
                meta.name.as_deref().unwrap_or("未定义"),
                meta.version.as_deref().unwrap_or("未定义"),
                meta.platform.as_deref().unwrap_or("未定义"),
                meta.author.as_deref().unwrap_or("未定义"),
                meta.description.as_deref().unwrap_or("未定义"),
            ]);
            print_info_format!("{}", table.to_string().blue())
        }
        Err(e) => {
            print_error_format!("{}{}", "读取模块元数据时出错: ".red(), e.to_string().red());
            table.add_row(vec!["未定义", "未定义", "未定义", "未定义", "未定义"]);
            print_error_format!("{}", table.to_string().red());
        }
    }
}