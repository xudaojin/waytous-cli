/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: autocompletion
 */
use std::env;
use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::Bash;
use clap_complete::Shell::Zsh;

use crate::waytous;

pub struct GenerateAutoCompletion;

impl GenerateAutoCompletion {
    // 生成命令行工具的自动补全脚本
    pub fn process() {
        let mut app = waytous::Waytous::command();
        let out_dir = env::var_os("OUT_DIR").unwrap_or_else(|| PathBuf::from(".").into());
        generate_to(Bash, &mut app, "waytous", &out_dir).expect("ailed to generate bash completion");
        generate_to(Zsh, &mut app, "waytous", &out_dir).expect("ailed to generate zsh completion");
    }
}