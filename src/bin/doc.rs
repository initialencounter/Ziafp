#![windows_subsystem = "windows"]
use copypasta::{ClipboardContext, ClipboardProvider};
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use is_elevated::is_elevated;
use lazy_static::lazy_static;
use native_dialog::{MessageDialog, MessageType};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::env;
use std::io;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

lazy_static! {
    static ref PROJECT_NO_REGEX: Regex = Regex::new(r"[P|S]EK.{2}\d{12}").unwrap();
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

fn check_project_no(project_no: &str) -> bool {
    return PROJECT_NO_REGEX.is_match(project_no);
}

fn simulate_f5_press() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::F5, Click).unwrap();
}

async fn send_task(request_body: &serde_json::Value) -> Result<()> {
    let client = Client::new();

    // 发送POST请求
    let response = client
        .post("http://localhost:25457/edit-doc")
        .json(&request_body)
        .send()
        .await?;

    // 检查响应状态
    if response.status().is_success() {
        let res: EditDocResponse = response.json().await?;
        let save_path = res.save_path;
        open_file_with_default_program(&save_path);
    } else {
        println!("Failed to edit document: {:?}", response.text().await?);
    }

    Ok(())
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

fn get_clip_text() -> String {
    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    let clip_text = ctx.get_contents().unwrap();
    return clip_text;
}

fn popup_message(title: &str, message: &str) -> bool {
    let result = MessageDialog::new()
        .set_title(title)
        .set_text(&message)
        .set_type(MessageType::Warning)
        .show_confirm();
    result.unwrap()
}

async fn get_project_info(project_no: &str) -> Result<QueryResult> {
    let client = Client::new();

    // 发送POST请求
    let response = client
        .get(format!(
            "http://localhost:25455/get-project-info/{}",
            project_no
        ))
        .send()
        .await
        .unwrap();

    // 检查响应状态
    if response.status().is_success() {
        let res: QueryResult = response.json().await?;
        Ok(res)
    } else {
        Err("未找到项目信息".into())
    }
}

fn open_file_with_default_program(path: &str) {
    Command::new("cmd")
        .args(&["/C", "start", "", path])
        .spawn()
        .expect("Failed to open file with default program");
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
        let key_name = "doc";
        let menu_name = "Edit doc";
        let command = format!(r#""{}" "%V""#, current_exe_abs_path);
        let icon = format!(r#"{}"#, current_exe_abs_path);
        create_reg(key_name, menu_name, &command, &icon).unwrap();
        return;
    }

    let target_dir = args[1].to_string();
    let clip_text = get_clip_text();
    if !check_project_no(&clip_text) {
        popup_message(
            "项目编号不合法",
            &format!("请检查项目编号是否正确: {}", clip_text),
        );
        return;
    }
    println!("项目编号: {}", clip_text);

    match get_project_info(&clip_text).await {
        Ok(project_info) => {
            let item_c_name = project_info.rows[0].item_c_name.clone();
            let item_e_name = project_info.rows[0].item_e_name.clone();
            let is_965 = if item_c_name.contains("内置") || item_c_name.contains("包装") {
                false
            } else {
                true
            };
            let is_power_bank =
                item_c_name.contains("移动电源") || item_c_name.contains("储能电源");
            // 构建请求体
            let request_body = json!({
                "source_path": env::current_exe().unwrap().parent().unwrap().join("image.doc").to_str().unwrap(),
                "save_dir": target_dir,
                "project_no": clip_text,
                "project_name": item_c_name,
                "is_965": is_965,
                "is_power_bank": is_power_bank,
                "en_name": item_e_name.split(" ").nth(1).unwrap()
            });
            send_task(&request_body).await.unwrap();
            simulate_f5_press();
        }
        _ => {
            popup_message("项目信息获取失败", "请检查项目编号是否正确");
        }
    }
}
