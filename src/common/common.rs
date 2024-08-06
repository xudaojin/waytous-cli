/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-31
 * File: common
 */

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::string::String;

use serde::{Deserialize, Serialize};
use toml;

use crate::common::common;

pub mod module_constants {
    pub const INSTALL_PATH: &str = "/opt/waytous/modules";
    pub const MODULE_META_DATA_NAME: &str = "version.toml";
}

/// 打印 info 信息
#[macro_export]
macro_rules! print_info_msg {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.green());
    }};
}

/// 打印 error 信息
#[macro_export]
macro_rules! print_error_msg {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.red());
    }};
}

/// 打印 warning 信息
#[macro_export]
macro_rules! print_warning_msg {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.yellow());
    }};
}
#[macro_export]
macro_rules! print_debug_msg {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.blue());
    }};
}

#[macro_export]
macro_rules! print_tips_msg {
    ($($arg:tt)*) => {{
        use colored::Colorize; // 在宏内部导入 Colorize trait
        let formatted_string = format!($($arg)*); // 使用 format! 直接生成字符串
        println!("{}", formatted_string.cyan());
    }};
}

/// 操作系统信息
pub struct SystemInfo {

    #[allow(dead_code)]
    /// 主机名
    pub hostname: String,

    #[allow(dead_code)]

    ///操作系统版本代号 (ex: "focal", "bionic")
    pub code_name: String,

    #[allow(dead_code)]

    ///操作系统版本号(ex: "20.04", "18.04")
    pub release: String,

    #[allow(dead_code)]

    /// 系统架构(ex: "amd64", "arm64")
    pub architecture: String,

    #[allow(dead_code)]

    /// 系统内核版本
    pub kernel_version: String,
}

/// 已安装的模块信息
#[derive(Serialize, Deserialize, Default)]
pub struct ModuleInfo {
    /// 软件模块的安装路径
    pub install_path: String, // 模块安装路径

    /// 软件模块的包大小
    pub module_file_size: u64,

    /// 软件模块包含的文件数
    pub module_file_num: u64, // 模块文件数量

    /// 软件模块版本信息
    pub version_meta_data: VersionMetaData, // 模块元数据
}

/// 模块的元数据
#[derive(Serialize, Deserialize, Default)]
pub struct VersionMetaData {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub author: String,
    pub description: String,
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
                print_error_msg!("{}", &String::from_utf8(output.stderr).unwrap());
                false // 返回 false，指示命令执行失败
            }
        }
        Err(e) => {
            print_error_msg!("无法执行命令: {}\n错误信息: {}", program, e);
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
        }
        Err(e) => {
            format!(
                "错误信息:{}\n没有安装{}, 请先安装 {} 后再次执行.",
                e, program, program
            )
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
    let code_name = system_ret("lsb_release", vec!["-c", "-s"])
        .to_string()
        .replace("\n", "");
    let release = system_ret("lsb_release", vec!["-r", "-s"])
        .to_string()
        .replace("\n", "");
    let architecture = system_ret("uname", vec!["-m"])
        .to_string()
        .replace("\n", "");
    let hostname = system_ret("hostname", vec!["-s"])
        .to_string()
        .replace("\n", "");
    let kernel_version = system_ret("uname", vec!["-r"])
        .to_string()
        .replace("\n", "");
    SystemInfo {
        code_name,
        release,
        architecture,
        hostname,
        kernel_version,
    }
}

/// 根据传入的模块名称，返回该模块的信息
/// # Arguments
/// * `name` - 模块名称
/// # Return
/// ModuleInfo - 模块信息
pub fn get_installed_module_info(name: &str) -> Result<ModuleInfo, String> {
    // 根据传入的模块名称， 组装模块的安装路径
    let module_install_path = format!("{}/{}", module_constants::INSTALL_PATH, name);

    // 获取传入模块名称的模块元数据文件路径
    let module_meta_file_path = format!(
        "{}/{}/{}",
        module_constants::INSTALL_PATH,
        name,
        module_constants::MODULE_META_DATA_NAME
    );

    // 检查传入的模块是否存在
    match fs::metadata(module_install_path) {
        Ok(_file) => {
            let path = Path::new(&module_meta_file_path);

            // 检查模块元数据文件是否存在
            if path.is_file() && path.exists() {
                let mut contents = String::new();

                File::open(module_meta_file_path.to_string())
                    .unwrap()
                    .read_to_string(&mut contents)
                    .unwrap();
                let metadata: VersionMetaData = toml::from_str(&contents).unwrap();
                let module_info = ModuleInfo {
                    install_path: "".to_string(),
                    module_file_size: 0,
                    module_file_num: 0,
                    version_meta_data: metadata,
                };
                Ok(module_info)
            } else {
                let module_info = ModuleInfo {
                    install_path: "".to_string(),
                    module_file_size: 0,
                    module_file_num: 0,
                    version_meta_data: Default::default(),
                };
                Ok(module_info)
            }
        }
        Err(e) => {
            return Err(format!("{}", e));
        }
    }
}

/// 获取已经安装的模块列表
pub fn get_installed_module_list() -> Vec<String> {
    let mut module_list = Vec::new();

    // 读取目录的内容
    if let Ok(entries) = fs::read_dir(common::module_constants::INSTALL_PATH) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                // 检查条目是否是目录
                if path.is_dir() {
                    // 获取目录名字并存储到列表中
                    if let Some(folder_name) = path.file_name() {
                        if let Some(folder_name_str) = folder_name.to_str() {
                            module_list.push(folder_name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    module_list
}
