/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-31
 * File: common
 */


use std::process::Command;
use colored::Colorize;
use rusqlite::{params, Connection, Result, Error};


/// 执行 shell 终端指令 并返回执行状态
/// # Arguments
/// * `program` - 指令名称
/// * `parameter` - 参数列表
///
/// # Return
/// true - 执行成功
/// false - 执行失败
pub fn system(program: &str, parameter: Vec<&str>) -> bool {
    let output = Command::new(program)
        .args(&parameter)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap().to_string().blue());
        true
    }else {
        println!("{}", String::from_utf8(output.stderr).unwrap().to_string().red());
        false
    }
}

/// 执行 shell 终端指令 并返回执行结果
pub fn system_ret(program: &str, parameter: Vec<&str>) -> String {
    let output = Command::new(program)
        .args(&parameter)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        String::from_utf8(output.stdout).unwrap().to_string()
    }else {
        String::from_utf8(output.stderr).unwrap().to_string()
    }

}

/// 获取系统信息
/// * `code_name` - 发行版代号, 例如: bionic focal
/// * `release` - 发行版版本, 例如: 18.04 20.04
/// * `architecture` - 系统架构, 例如: x86_64 aarch64
/// * `hostname` - 主机名
pub struct SystemInfo {
    pub code_name: String,
    pub release: String,
    pub architecture: String,
    pub hostname: String,
}


/// 获取系统信息
/// # Arguments
/// * null
///
/// # Return
/// SystemInfo - 系统信息
pub fn get_system_info() -> SystemInfo {
    let  code_name = system_ret("lsb_release", vec!["-c", "-s"]);
    let  release = system_ret("lsb_release", vec!["-r", "-s"]);
    let  architecture = system_ret("uname", vec!["-m"]);
    let  hostname = system_ret("hostname", vec!["-s"]);

    SystemInfo {
        code_name,
        release,
        architecture,
        hostname,
    }
}


/// 模块的元数据
#[derive(Debug)]
pub struct ModuleMetaDate {
    pub id: i32,
    pub name: Option<String>,
    pub version: Option<String>,
    pub platform: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

/// 读取 当前模块的的 meta 数据
/// # Arguments
/// * null
/// # Return
/// * ModuleMetaDate - 模块的元数据
/// 读取当前模块的元数据
/// # Return
/// * Result<ModuleMetaDate> - 模块的元数据
pub fn get_module_meta() -> Result<ModuleMetaDate, Error> {
    // 数据库连接
    let conn = Connection::open("meta")?;

    // 查询数据
    let mut stmt = conn.prepare("SELECT id, name, version, platform, author, description FROM meta")?;
    let mut meta_iter = stmt.query_map([], |row| {
        Ok(ModuleMetaDate {
            id: row.get(0)?,
            name: row.get(1).ok(),
            version: row.get(2).ok(),
            platform: row.get(3).ok(),
            author: row.get(4).ok(),
            description: row.get(5).ok(),
        })
    })?;

    // 返回数据
    if let Some(meta) = meta_iter.next() {
        meta
    } else {
        Err(Error::QueryReturnedNoRows)
    }
}
