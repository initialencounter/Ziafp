#![windows_subsystem = "windows"]
use std::env;
use std::io;


use reqwest::blocking::Client;
use winreg::enums::*;
use winreg::RegKey;

#[derive(serde::Deserialize, serde::Serialize)]
struct DirectoryInfo {
    dir: String,
}

fn create_reg(key_name: &str, menu_name: &str, command: &str, icon: &str) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let vs_code_key = hklm.create_subkey(format!("SOFTWARE\\Classes\\Directory\\background\\shell\\{}", key_name))?;
    vs_code_key.0.set_value("", &menu_name)?;
    vs_code_key.0.set_value("Icon", &icon)?;
    let sub_command = vs_code_key.0.create_subkey("command")?;
    sub_command.0.set_value("", &command)?;
    println!("注册表项已创建");
    Ok(())
}

pub fn post_file(file_path: String) -> () {
    let client = Client::new();
    let body = DirectoryInfo {
        dir: file_path,
    };
    let response = client.post("http://127.0.0.1:25455/upload")
        .json(&body)
        .send().unwrap();

    if response.status().is_success() {
        ()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let current_exe = env::current_exe().expect("无法获取当前执行文件路径");
        let current_exe_abs_path = current_exe.to_str().expect("无法获取当前执行文件路径");
        let key_name = "limsClient";
        let menu_name = "Upload file here";
        let command = format!(r#""{}" %V"#, current_exe_abs_path);
        create_reg(key_name, menu_name, &command, "").unwrap();
        return;
    }
    
    if args[1] == "code" {
        let key_name = "AA";
        let menu_name = "Open VSCode Here";
        let command = r#""C:\Users\29115\AppData\Local\Programs\Microsoft VS Code\Code.exe" "%V""#;
        let icon = r#"C:\Users\29115\AppData\Local\Programs\Microsoft VS Code\Code.exe"#;
        create_reg(key_name, menu_name, command, icon).unwrap();
    }
    let _ = post_file(args[1].to_string());
}