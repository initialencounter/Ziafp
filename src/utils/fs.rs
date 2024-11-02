use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    static ref PROJECT_NO_REGEX: Regex = Regex::new(r"[P|S]EK.{2}\d{12}").unwrap();
}

#[derive(serde::Deserialize, Debug)]
pub struct RawFileInfo {
    pub file_name: String,
    pub project_no: String,
    pub file_path: PathBuf,
}

pub struct FileInfo {
    pub project_no: String,
    pub file_id: String,
    pub file_buffer: Vec<u8>,
    pub file_name: String,
    pub file_type: String,
}

pub fn get_file_buffer(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn match_file(dir: &PathBuf) -> Vec<RawFileInfo> {
    let mut file_list = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        // 检查文件名是否符合要求
        if !file_name.ends_with(".pdf")
            || (!file_name.starts_with("PEK") && !file_name.starts_with("SEK"))
        {
            continue;
        }

        // 构造对应的 doc 文件路径
        let doc_file_path = path.with_extension(if file_name.contains("概要") {
            "docx"
        } else {
            "doc"
        });
        // 如果 doc 源文件不存在，则说明 pdf 不是 doc 转换而来的，直接跳过
        if !doc_file_path.exists() {
            continue;
        }

        let project_no = if let Some(match_result) = PROJECT_NO_REGEX.find(&file_name) {
            match_result.as_str().to_string()
        } else {
            continue;
        };
        file_list.push(RawFileInfo {
            file_name,
            project_no,
            file_path: path,
        });
    }
    file_list
}

pub fn prepare_file_info(file_info: RawFileInfo) -> Option<FileInfo> {
    let path = file_info.file_path;
    let file_name = path.file_name()?.to_str()?.to_string();

    // 获取 pdf 文件大小
    let file_size = fs::metadata(&path).ok()?.len();

    // 获取文件内容
    let file_buffer = get_file_buffer(&path).ok()?;

    // 构造 file_id
    let file_id = format!("{}_{}", file_size, file_name.replace(" ", "_"));

    // 确定文件类型
    let file_type = if file_name.contains("概要") {
        "batteryfile"
    } else {
        "goodsfile"
    }
    .to_string();
    Some(FileInfo {
        project_no: file_info.project_no,
        file_id,
        file_buffer,
        file_name,
        file_type,
    })
}
