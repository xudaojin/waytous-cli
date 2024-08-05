/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: autocompletion
 */
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::Bash;

use crate::{cli, print_info_msg};

pub struct GenerateAutoCompletion;

impl GenerateAutoCompletion {
    // 生成命令行工具的自动补全脚本
    pub fn process() {
        let mut app = cli::Cli::command();

        // 获取用户目录
        let home_dir = env::var("HOME").expect("failed to get user home dir");

        // 定义.bashrc 文件路径
        let bashrc_file_path = PathBuf::from(&home_dir).join(".bashrc");

        // 定义自动补全脚本路径
        let autocompletion_file_path = PathBuf::from(&home_dir).join(".bash_completion.d/waytous.bash");

        // 定义自动补全脚本的存放目录
        let autocompletion_dir = PathBuf::from(&home_dir).join(".bash_completion.d");

        // 定义自动补全脚本的引用命令
        let autocompletion_line = format!("source {:?}", autocompletion_file_path);

        // 检查目录是否存在，不存在则创建
        if !autocompletion_dir.exists() {
            fs::create_dir(&autocompletion_dir).expect("failed to create completion dir");
        }

        // 开始生成 自动补全脚本
        print_info_msg!("开始生成自动补全脚本...");
        generate_to(Bash, &mut app, "waytous", &autocompletion_dir).expect("ailed to generate bash completion");

        // 检查 .bashrc 文件内是否包含的自动补全脚本的引用命令, 存在则不写入
        if !GenerateAutoCompletion::is_line_in_file(&bashrc_file_path, &autocompletion_line) {
            let mut bashrc_file = OpenOptions::new()
                .append(true)
                .open(&bashrc_file_path)
                .expect("failed to open .bashrc");

            writeln!(bashrc_file, "\n{}\n", autocompletion_line).expect("failed to write bashrc");
        }
        print_info_msg!("配置完成, 请执行 source ~/.bashrc 命令 或重启终端使=自动补全生效");
    }

    // 检查文件中是否包含指定的行
    fn is_line_in_file(file_path: &PathBuf, line_to_check: &str) -> bool {
        if let Ok(file) = fs::File::open(file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim() == line_to_check.trim() {
                        return true;
                    }
                }
            }
        }
        false
    }
}