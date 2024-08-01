/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: module
 */

use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use crate::{subcommand_define};
use rusqlite::{params, Connection, Result};
use comfy_table::{ContentArrangement, Table};
use crate::common;
pub fn module(sub_cmd: &subcommand_define::ModuleCmds) {
    match sub_cmd {
        subcommand_define::ModuleCmds::Create { name, no_demo } => { create(name, *no_demo); }
        subcommand_define::ModuleCmds::Build { jobs } => { build(jobs); }
        subcommand_define::ModuleCmds::Set { name, version, platform, author, description } => { set_config(name, version, platform, author, description).expect("TODO: panic message"); }
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
fn get_installed_module_list() {

}
fn create(name: &str, no_demo: bool) {
        println!("{}", no_demo)
}
fn build(jobs: &u32) {}
fn set_config(name: &Option<String>, version: &Option<String>, platform: &Option<String>, author: &Option<String>, description: &Option<String>) -> Result<()> {
    let conn = Connection::open("meta")?;

    // 检查meta 是否存在，如果不存在则创建
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meta (
        id INTEGER PRIMARY KEY,
        name TEXT UNIQUE,
        version TEXT,
        platform TEXT,
        author TEXT,
        description  NULL,
        UNIQUE(name)
    )",
        [],
    )?;

    /// 检查 meta 是否为空, 如果为空， 则对其初始化
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM meta", [], |r| r.get(0))?;
    if count == 0 {

        match name {
            Some(name) => {
                conn.execute("INSERT INTO meta (name) VALUES (?1)", params![name])?;
            }
            None => {
            }
        }

        match version {
            Some(version) => {
                conn.execute("INSERT INTO meta (version) VALUES (?1)", params![version])?;
            }
            None => {}
        }

        match platform {
            Some(platform) => {
                conn.execute("INSERT INTO meta (platform) VALUES (?1)", params![platform])?;
            }
            None => {}
        }

        match author {
            Some(author) => {
                conn.execute("INSERT INTO meta (author) VALUES (?1)", params![author])?;
            }
            None => {}
        }

        match description {
            Some(description) => {
                conn.execute("INSERT INTO meta (description) VALUES (?1)", params![description])?;
            }
            None => {}
        }

    } else {
        if let Some(name) = name {
            conn.execute("UPDATE meta SET name = ?1 WHERE id = 1", params![name])?;
        }
        if let Some(version) = version {
            conn.execute("UPDATE meta SET version = ?1 WHERE id = 1", params![version])?;
        }
        if let Some(platform) = platform {
            conn.execute("UPDATE meta SET platform = ?1 WHERE id = 1", params![platform])?;
        }
        if let Some(author) = author {
            conn.execute("UPDATE meta SET author = ?1 WHERE id = 1", params![author])?;
        }
        if let Some(description) = description {
            conn.execute("UPDATE meta SET description = ?1 WHERE id = 1", params![description])?;
        }
    }

    Ok(())
}
fn get_config_info() {
    match common::common::get_module_meta() {
        Ok(meta) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["模块名称", "版本号", "适用平台", "作者", "描述"])
                .add_row(vec![
                    meta.name.as_deref().unwrap_or("未定义"),
                    meta.version.as_deref().unwrap_or("未定义"),
                    meta.platform.as_deref().unwrap_or("未定义"),
                    meta.author.as_deref().unwrap_or("未定义"),
                    meta.description.as_deref().unwrap_or("未定义"),
                ]);

            println!("{}", table.to_string().blue())
        }
        Err(e) => {
            println!("{}{}", "读取模块元数据时出错: ".red(), e.to_string().red() )
        }
    }

}