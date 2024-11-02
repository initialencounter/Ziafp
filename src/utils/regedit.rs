use std::env;
use std::io;
use std::process::Command;

use is_elevated::is_elevated;
use winreg::enums::*;
use winreg::RegKey;

use crate::utils::dialog::popup_message;

pub fn create_auto_run_reg(key_name: &str, command: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let auto_run_key = hkcu.create_subkey(r#"Software\MIcrosoft\Windows\CurrentVersion\Run"#)?;
    let command_str = format!(r#""{}" --from-registry /background"#, command);
    println!("{}", command_str);
    auto_run_key.0.set_value(key_name, &command_str)?;
    Ok(())
}

// 如果当前没有管理员权限，则重新以管理员身份启动程序, 判断是否需要退出
pub fn request_admin_and_restart() -> bool {
    if is_elevated() {
        return false;
    }

    // 如果没有管理员权限，则重新以管理员身份启动程序
    let current_exe = env::current_exe().expect("无法获取当前执行文件路径");
    let status = Command::new("powershell")
        .arg("Start-Process")
        .arg("-Verb")
        .arg("RunAs")
        .arg(current_exe)
        .status()
        .expect("无法启动管理员进程");

    if !status.success() {
        popup_message("需要管理员权限", "需要管理员权限才能修改注册表");
    }
    true
}

pub fn is_launched_from_registry() -> bool {
    std::env::args().any(|arg| arg == "--from-registry")
}
