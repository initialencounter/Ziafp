use std::io;

use winreg::enums::*;
use winreg::RegKey;

pub fn create_auto_run_reg(key_name: &str, command: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let auto_run_key = hkcu.create_subkey(r#"Software\MIcrosoft\Windows\CurrentVersion\Run"#)?;
    let command_str = format!(r#""{}" --from-registry /background"#, command);
    println!("{}", command_str);
    auto_run_key.0.set_value(key_name, &command_str)?;
    Ok(())
}
