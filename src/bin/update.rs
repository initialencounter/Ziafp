#![windows_subsystem = "windows"]
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use copypasta::{ClipboardContext, ClipboardProvider};
use is_elevated::is_elevated;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;


use chrono::Local;
use serde_json::json;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROJECT_NO_REGEX: Regex = Regex::new(r"[P|S]EK.{2}\d{12}").unwrap();
}
use native_dialog::{MessageDialog, MessageType};

use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

#[derive(serde::Deserialize, Debug)]
pub struct RawFileInfo {
    pub file_name: String,
    pub project_no: String,
    pub file_path: PathBuf,
}

fn popup_message(title: &str, message: &str) -> bool {
    let result = MessageDialog::new()
        .set_title(title)
        .set_text(&message)
        .set_type(MessageType::Warning)
        .show_confirm();
    result.unwrap()
}

#[derive(Serialize)]
struct SearchParams {
    search: String,
    json: i32,
    path_column: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    path: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

pub fn search(file_path: String) -> Vec<SearchResult> {
    let client = Client::new();
    let query = SearchParams {
        search: file_path,
        json: 1,
        path_column: 1,
    };
    let response = client
        .get("http://127.0.0.1:25456")
        .query(&query)
        .send()
        .unwrap();

    if response.status().is_success() {
        let text = response.text().unwrap();
        println!("Response Text: {}", text);

        // 使用新的结构体解析 JSON
        let result: SearchResponse = serde_json::from_str(&text).unwrap();
        return result.results;
    }
    return vec![];
}

fn simulate_f5_press() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::F5, Click).unwrap();
}

fn copy_to_here(search_result: Vec<SearchResult>, target_path: String) -> () {
    let mut file_list = vec![];
    for result in search_result {
        let source_path = format!("{}\\{}", result.path, result.name);
        if result.name.is_empty() {
            continue;
        }
        if result.name.ends_with(".doc") || result.name.ends_with(".docx") {
            file_list.push(source_path);
        }
    }

    if !popup_message("确认复制文件?", &file_list.join("\n")) {
        return;
    }
    for source_path in file_list {
        let target_path = target_path.clone() + "\\" + &source_path.split("\\").last().unwrap();
        if let Err(e) = fs::copy(&source_path, &target_path) {
            eprintln!("Failed to copy {} to {}: {}", source_path, target_path, e);
        }
    }

    // 复制完成后模拟按下 F5
    simulate_f5_press();
}

fn get_clip_text() -> String {
    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    let clip_text = ctx.get_contents().unwrap();
    return clip_text;
}

fn create_reg(key_name: &str, menu_name: &str, command: &str, icon: &str) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let vs_code_key = hklm.create_subkey(format!(
        "SOFTWARE\\Classes\\Directory\\background\\shell\\{}",
        key_name
    ))?;
    vs_code_key.0.set_value("", &menu_name)?;
    vs_code_key.0.set_value("Icon", &icon)?;
    let sub_command = vs_code_key.0.create_subkey("command")?;
    sub_command.0.set_value("", &command)?;
    println!("注册表项已创建");
    Ok(())
}

fn check_project_no(project_no: &str) -> bool {
    return PROJECT_NO_REGEX.is_match(project_no);
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize, Serialize)]
struct QueryResult {
    rows: Vec<ProjectRow>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectRow {
    item_c_name: String,
    item_e_name: String,
    edit_status: i64,
    project_id: String,
    project_no: String,
}

#[derive(Deserialize, Serialize)]
struct EditDocResponse {
    message: String,
    save_path: String,
}


async fn send_task(path: String, project_no: &str) -> Result<()> {
    let client = Client::new();
    let today_date = get_today_date();
    let request_body = json!({
        "source_path": &path,
        "project_no": project_no,
        "date": today_date,
        "signature_img_path": env::current_exe().unwrap().parent().unwrap().join("signature.png").to_str().unwrap()

    });

    // 发送POST请求
    let response = client
        .post("http://localhost:25457/edit-docx")
        .json(&request_body)
        .send()?;

    // 检查响应状态
    if response.status().is_success() {
        let _res: EditDocResponse = response.json()?;
        open_file_with_default_program(&path);
    } else {
        popup_message("替换文件失败", "替换文件失败");
    }

    Ok(())
}

fn open_file_with_default_program(path: &str) {
    Command::new("cmd")
        .args(&["/C", "start", "", path])
        .spawn()
        .expect("Failed to open file with default program");
}

async fn match_file(dir: &PathBuf, project_no: &str) {
    let mut file_path_list = vec![];
    let mut file_name_list = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !file_name.contains("概要") {
            continue;
        }
        // 检查文件名是否符合要求
        if !file_name.ends_with(".docx") {
            continue;
        }
        if !file_name.starts_with("PEK") && !file_name.starts_with("SEK") {
            continue;
        }
        file_path_list.push(path.to_str().unwrap().to_string());
        file_name_list.push(file_name.to_string());
    }
    for path in file_path_list {
        let _ = send_task(path, project_no).await;
    }
}

fn get_today_date() -> String {
    // 获取当前日期
    let today = Local::now().naive_local().date();

    // 格式化为 YYYY-MM-DD
    let formatted_date = today.format("%Y-%m-%d").to_string();
    formatted_date
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        if !is_elevated() {
            // 如果没有管理员权限，则重新以管理员身份启动程序
            let current_exe = env::current_exe().expect("无法获取当前执行文件路径");
            let status = Command::new("powershell")
                .arg("Start-Process")
                .arg("-Verb")
                .arg("RunAs")
                .arg(current_exe)
                .status()
                .expect("无法启动管理员进程");

            if status.success() {
                return;
            } else {
                println!("需要管理员权限才能修改注册表");
                return;
            }
        }
        let current_exe = env::current_exe().expect("无法获取当前执行文件路径");
        let current_exe_abs_path = current_exe.to_str().expect("无法获取当前执行文件路径");
        let key_name = "docx";
        let menu_name = "Replace ALL docx";
        let command = format!(r#""{}" "%V""#, current_exe_abs_path);
        let icon = format!(r#"{}"#, current_exe_abs_path);
        create_reg(key_name, menu_name, &command, &icon).unwrap();
        return;
    }

    let target_dir = args[1].to_string();
    let clip_text = get_clip_text();
    if !check_project_no(&clip_text) {
        popup_message("项目编号不合法", "请检查项目编号是否正确");
        return;
    }
    let search_result = search(clip_text.clone());
    copy_to_here(search_result, target_dir.clone());
    match_file(&PathBuf::from(&target_dir), &clip_text).await;
}
