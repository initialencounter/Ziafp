use std::env;
use std::io;
use std::process::Command;

use is_elevated::is_elevated;

use crate::utils::dialog::popup_message;

pub fn is_launched_from_powershell() -> bool {
    std::env::args().any(|arg| arg == "--from-powershell")
}

pub fn restart_with_powershell() -> io::Result<()> {
    let current_exe = env::current_exe().expect("无法获取当前执行文件路径");
    let status = Command::new("powershell")
        .arg("Start-Process")
        .arg("-Verb")
        .arg("Open")
        .arg(format!(
            r#""{}" --from-powershell"#,
            current_exe.to_str().unwrap()
        ))
        .status()
        .expect("无法使用powershell启动");
    if !status.success() {
        popup_message("无法使用powershell启动", "无法使用powershell启动");
    }
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
