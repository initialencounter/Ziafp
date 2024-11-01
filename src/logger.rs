use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct Logger {
    enabled: bool,
    file: File,
}

impl Logger {
    pub fn new(log_dir: PathBuf, service_name: &str, enabled: bool) -> Self {
        if !enabled {
            return Logger {
                file: File::create("NUL").unwrap(),
                enabled,
            };
        }
        std::fs::create_dir_all(&log_dir).expect("无法创建日志目录");

        let log_path = log_dir.join(format!("{}.log", service_name));
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("无法打开日志文件");

        Logger { file, enabled }
    }

    pub fn log(&mut self, level: &str, message: &str) {
        let now = Local::now();
        let log_entry = format!(
            "[{}] {} - {}\n",
            now.format("%Y-%m-%d %H:%M:%S"),
            level,
            message
        );
        if self.enabled {
            self.file
                .write_all(log_entry.as_bytes())
                .expect("写入日志失败");
        }
        println!("{}", log_entry);
    }
}
