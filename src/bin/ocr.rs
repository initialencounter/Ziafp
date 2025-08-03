// #![windows_subsystem = "windows"]
use std::env;
use std::io;
use std::path::PathBuf;

use is_elevated::is_elevated;
use std::process::Command;
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

use native_dialog::{FileDialog, MessageDialog, MessageType};

pub fn popup_message(title: &str, message: &str) -> bool {
    let result = MessageDialog::new()
        .set_title(title)
        .set_text(&message)
        .set_type(MessageType::Warning)
        .show_confirm();
    result.unwrap()
}

fn get_ocrmypdf_path() -> Option<String> {
    let result = FileDialog::new()
        .set_title("选择 OCRmyPDF 可执行文件")
        .add_filter("可执行文件", &["exe"])
        .show_open_single_file();

    match result {
        Ok(Some(path)) => Some(path.to_string_lossy().to_string()),
        Ok(None) => None,
        Err(_) => None,
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct DirectoryInfo {
    dir: String,
}

fn create_reg(key_name: &str, menu_name: &str, command: &str, icon: &str) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
    let vs_code_key = hklm.create_subkey(format!("KWPS.PDF.9\\shell\\{}", key_name))?;
    vs_code_key.0.set_value("", &menu_name)?;
    vs_code_key.0.set_value("Icon", &icon)?;
    let sub_command = vs_code_key.0.create_subkey("command")?;
    sub_command.0.set_value("", &command)?;
    println!("注册表项已创建");
    Ok(())
}

pub fn ocr_my_pdf(ocrmypdf_exe_path: String, file_path: String) -> () {
    let path = PathBuf::from(file_path.clone());
    let output_path = path.parent().unwrap().join("ocr.pdf");
    println!(
        "执行命令: {} \"{}\" \"{}\" --skip-text",
        ocrmypdf_exe_path,
        file_path,
        output_path.display()
    );
    let status = Command::new(&ocrmypdf_exe_path)
        .arg(&file_path)
        .arg(&output_path)
        .arg("--skip-text")
        .status()
        .expect("无法启动 OCRmyPDF 进程");
    if status.success() {
        return;
    } else {
        popup_message("错误", "OCRmyPDF 执行失败，请确保已安装 OCRmyPDF。");
        return;
    }
}

fn main() {
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
        let key_name = "ocrmypdf";
        let menu_name = "OCRmyPDF";
        // 获取用户输入的 OCRmyPDF 可执行文件路径
        let ocrmypdf_exe_path = match get_ocrmypdf_path() {
            Some(path) => path,
            None => {
                popup_message("错误", "未选择 OCRmyPDF 可执行文件路径");
                r"ocrmypdf.exe".to_string()
            }
        };
        let command = format!(r#""{}" "{}" "%1""#, current_exe_abs_path, ocrmypdf_exe_path);
        let icon = format!(r#"{}"#, current_exe_abs_path);
        create_reg(key_name, menu_name, &command, &icon).unwrap();
        return;
    }
    let _ = ocr_my_pdf(args[1].to_string(), args[2].to_string());
}
