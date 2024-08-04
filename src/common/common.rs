/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-31
 * File: common
 */


use std::process::Command;
use std::string::String;

use rusqlite::{Connection, Error, Result};

/// 软件包元数据名称
pub static MODULE_META_DATA_DB: &str = "package.db";


/// 打印 info 信息
#[macro_export]
macro_rules! print_info_format {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.blue());
    }};
}

/// 打印 error 信息
#[macro_export]
macro_rules! print_error_format {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.red());
    }};
}

/// 打印 warning 信息
#[macro_export]
macro_rules! print_warning_format {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.yellow());
    }};
}

/// 获取系统信息
pub struct SystemInfo {
    pub code_name: String,      // 发行版代号
    pub release: String,        // 发行版版本
    pub architecture: String,   // 系统架构
    pub hostname: String,       // 主机名
    pub kernel_version: String,         // 内核版本

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

/// 已安装的自动驾驶模块信息
pub struct InstalledModuleInfo {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub install_path: String,
}


/// 执行 shell 终端指令 并返回执行状态
/// # Arguments
/// * `program` - 指令名称
/// * `parameter` - 参数列表
///
/// # Return
/// true - 执行成功
/// false - 执行失败
pub fn system(program: &str, parameter: Vec<&str>) -> bool {
    // 尝试执行命令
    let output = Command::new(program).args(&parameter).output();

    match output {
        Ok(output) => {
            // 检查命令是否成功执行
            if output.status.success() {
                true // 返回 true，指示命令执行成功
            } else {
                // 打印标准错误，并使用红色
                print_error_format!("{}", &String::from_utf8(output.stderr).unwrap());
                false // 返回 false，指示命令执行失败
            }
        },
        Err(e) => {
            print_error_format!("无法执行命令: {}\n错误信息: {}", program, e);
            false // 返回 false，指示命令执行失败
        }
    }
}

/// 执行 shell 终端指令 并返回执行结果
pub fn system_ret(program: &str, parameter: Vec<&str>) -> String {
    let output = Command::new(program).args(&parameter).output();
    match output {
        Ok(output) => {
            return if output.status.success() {
                String::from_utf8(output.stdout).unwrap().to_string()
            } else {
                String::from_utf8(output.stderr).unwrap().to_string()
            }
        },
        Err(e) => {
            format!("错误信息:{}\n没有安装{}, 请先安装 {} 后再次执行.", e, program, program)
        }
    }
}

/// 获取系统信息
/// # Arguments
/// * null
///
/// # Return
/// SystemInfo - 系统信息
pub fn get_system_info() -> SystemInfo {
    let code_name = system_ret("lsb_release", vec!["-c", "-s"]);
    let release = system_ret("lsb_release", vec!["-r", "-s"]);
    let architecture = system_ret("uname", vec!["-m"]);
    let hostname = system_ret("hostname", vec!["-s"]);
    let kernel_version = system_ret("uname", vec!["-r"]);
    SystemInfo {
        code_name,
        release,
        architecture,
        hostname,
        kernel_version
    }
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
    let conn = Connection::open(MODULE_META_DATA_DB)?;

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
        Ok(meta?)
    } else {
        Err(Error::QueryReturnedNoRows)
    }
}
