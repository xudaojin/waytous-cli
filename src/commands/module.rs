/*
 * Author: daojin.xu101@gmail.com
 * Date: 24-7-30
 * File: module
 */

use comfy_table::{ContentArrangement, Table};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use rusqlite::{Connection, params, Result};
use std::fs::File;
use ring::{aead, error};
use serde::{Deserialize, Serialize};
use toml::to_string;
use std::io::{Write, Read};
use ring::aead::{Aad, AES_256_GCM, LessSafeKey, Nonce, UnboundKey};
use ring::rand::{SystemRandom, SecureRandom};
use toml;
use crate::{common, print_error_msg, print_info_msg, print_tips_msg};
use crate::subcommand_define;
use crate::subcommand_define::ConfigSubCmd;

pub fn module(sub_cmd: &subcommand_define::ModuleCmds) {
    match sub_cmd {
        subcommand_define::ModuleCmds::Create { name, no_demo } => { create(name, *no_demo); }
        subcommand_define::ModuleCmds::Build { jobs } => { build(jobs); }
        subcommand_define::ModuleCmds::List { .. } => { print_installed_module_list(); }
        subcommand_define::ModuleCmds::Config { sub_cmd } => {
            match sub_cmd {
                ConfigSubCmd::Set { name, version, platform, author, description } => {
                    let _ = set_current_module_config(name, version, platform, author, description);
                }
                ConfigSubCmd::Get {  } => {
                    get_current_module_config();
                }
            }
        }
        subcommand_define::ModuleCmds::Get { sub_cmd } => {
            match sub_cmd {
                subcommand_define::GetSubCmd::Info { .. } => {
                    get_config_info();
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaData {
    name: String,
    version: String,
    platform: String,
    author: String,
    description: String,
}

/// 获取当前系统中已经安装的自动驾驶系统模块列表
/// # Argment
/// * 'Null'
///  # Return
///  * 'Null'
fn print_installed_module_list() {
    print_info_msg!("TODO: print installed module list");
}

fn create(name: &str, no_demo: bool) {
    print_info_msg!("创建 {}, demo: {}", name, no_demo);
}

fn build(jobs: &u32) {
    print_info_msg!("{}", jobs);
}

/// 设置当前模块的配置信息
fn set_current_module_config(name: &Option<String>, version: &Option<String>, platform: &Option<String>, author: &Option<String>, description: &Option<String>)  -> Result<(), Box<dyn std::error::Error>>{
    // 创建一个随机密钥
    let rng = "1234567890";
    let mut key_bytes = [0u8; 32];
    rng.fill(&mut key_bytes).map_err(|_| "Failed to generate random key")?;
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| "Failed to create unbound key")?;
    let sealing_key = LessSafeKey::new(unbound_key);

    // 创建要加密的配置数据
    let config = MetaData {
        name: name.clone().unwrap_or_else(|| "my_app".to_string()),
        version: version.clone().unwrap_or_else(|| "1.0.0".to_string()),
        platform: platform.clone().unwrap_or_else(|| "x86_64".to_string()),
        author: author.clone().unwrap_or_else(|| "your_name".to_string()),
        description: description.clone().unwrap_or_else(|| "A sample application".to_string()),
    };

    // 将配置数据序列化成 TOML 字符串
    let toml_string = toml::to_string(&config)?;

    // 使用 AES-256-GCM 进行加密
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes).map_err(|_| "Failed to generate random nonce")?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let mut in_out = toml_string.into_bytes();
    in_out.extend_from_slice(&vec![0u8; AES_256_GCM.tag_len()]); // 为认证标签保留空间

    sealing_key.seal_in_place_append_tag(
        nonce,
        Aad::empty(),
        &mut in_out,
    ).map_err(|_| "Failed to seal in place")?;

    // 将加密后的数据写入文件
    let mut file = File::create("config.enc")?;
    file.write_all(&nonce_bytes)?; // 写入 nonce
    file.write_all(&in_out)?; // 写入加密数据和标签

    Ok(())
}

/// 获取当前模块的配置信息
fn get_current_module_config() {
    let mut file = File::open("config.enc")?;
    let mut nonce_bytes = [0u8; 12];
    file.read_exact(&mut nonce_bytes)?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, "1234567890")?;
    let sealing_key = LessSafeKey::new(unbound_key);

    let mut in_out = encrypted_data;
    let tag_len = AES_256_GCM.tag_len();
    let data_len = in_out.len() - tag_len;
    let data = &mut in_out[..data_len];
    let tag = &in_out[data_len..];

    sealing_key.open_in_place(nonce, Aad::empty(), data)
        .map_err(|_| "Decryption failed")?;

    Ok(data.to_vec())
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
            print_info_msg!("{}", table.to_string().blue())
        }
        Err(e) => {
            print_error_msg!("{}{}", "读取模块元数据时出错: ".red(), e.to_string().red());
            table.add_row(vec!["未定义", "未定义", "未定义", "未定义", "未定义"]);
            print_error_msg!("{}", table.to_string().red());
        }
    }
}