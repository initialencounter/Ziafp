use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{fs, io};

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::{multipart, Client};
use reqwest::header;

lazy_static! {
    static ref PROJECT_NO_REGEX: Regex = Regex::new(r"[P|S]EK.{2}\d{12}").unwrap();
}

struct FileInfo {
    project_no: String,
    file_id: String,
    file_buffer: Vec<u8>,
    file_name: String,
    file_type: String,
}

fn prepare_file_info(path: &PathBuf) -> Option<FileInfo> {
    let file_name = path.file_name()?.to_str()?.to_string();

    // 检查文件名是否符合要求
    if !file_name.ends_with(".pdf")
        || (!file_name.starts_with("PEK") && !file_name.starts_with("SEK"))
    {
        return None;
    }

    // 构造对应的 doc 文件路径
    let doc_file_path = path.with_extension(if file_name.contains("概要") {
        "docx"
    } else {
        "doc"
    });
    // 如果 doc 源文件不存在，则说明 pdf 不是 doc 转换而来的，直接跳过
    if !doc_file_path.exists() {
        return None;
    }

    // 获取 pdf 文件大小
    let file_size = fs::metadata(&path).ok()?.len();

    // 从文件名中提取项目编号
    let project_no = PROJECT_NO_REGEX.find(&file_name)?.as_str().to_string();

    // 获取文件内容
    let file_buffer = get_file_buffer(path).ok()?;

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
        project_no,
        file_id,
        file_buffer,
        file_name,
        file_type,
    })
}

fn traverse_directory(path: &Path, depth: usize, client: &HttpClient) {
    if depth > 3 {
        return;
    }
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry_path = entry.unwrap().path();
            parse_path(entry_path.clone(), client);
            traverse_directory(&entry_path, depth + 1, client);
        }
    }
}

fn parse_path(path: PathBuf, client: &HttpClient) {
    if let Some(file_info) = prepare_file_info(&path) {
        if let Ok(project_id) = client.get_project_id(&file_info.project_no) {
            client
                .post_file(
                    &project_id,
                    &file_info.file_id,
                    file_info.file_buffer,
                    &file_info.file_name,
                    &file_info.file_type,
                )
                .unwrap_or_else(|e| println!("上传失败: {}", e));
        }
    }
}

fn get_file_buffer(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn parse_date(date_text: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let numbers: String = date_text.chars().filter(|c| c.is_digit(10)).collect();

    if numbers.len() < 8 {
        return Err("文件名称错误".into());
    }

    let year = &numbers[0..4];
    let month = &numbers[4..6];
    let day = &numbers[6..8];

    if month == "00" {
        Ok((format!("{}-01-01", year), format!("{}-12-31", year)))
    } else if day == "00" {
        Ok((
            format!("{}-{}-01", year, month),
            format!("{}-{}-31", year, month),
        ))
    } else {
        Ok((
            format!("{}-{}-{}", year, month, day),
            format!("{}-{}-{}", year, month, day),
        ))
    }
}

#[derive(serde::Deserialize)]
struct QueryResult {
    rows: Vec<ProjectRow>,
}

#[derive(serde::Deserialize)]
struct ProjectRow {
    #[serde(rename = "projectId")]
    project_id: String,
    #[serde(rename = "editStatus")]
    edit_status: i8
}

struct HttpClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
}

impl HttpClient {
    fn new(base_url: String, username: String, password: String) -> Self {
        // 启用 cookie store
        let client = Client::builder().cookie_store(true).build().unwrap();

        HttpClient {
            client,
            base_url,
            username,
            password,
        }
    }

    fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(format!("{}/login", self.base_url))
            .header(
                "Host",
                self.base_url
                    .to_string()
                    .replace("http://", "")
                    .replace("https://", ""),
            )
            .header("Referer", self.base_url.to_string())
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(format!(
                "type=password&username={}&password={}",
                self.username, self.password
            ))
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err("登录失败".into())
        }
    }

    fn get_project_id(&self, project_no: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (start_date, end_date) = parse_date(project_no)?;
        let system_id = if project_no.starts_with("PEK") {
            "pek"
        } else {
            "pek"
        };

        let url = format!(
            "{}/rest/inspect/query?systemId={}&category=battery&projectNo={}&startDate={}&endDate={}&page=1&rows=10",
            self.base_url, system_id, project_no, start_date, end_date
        );
        let response = self
            .client
            .get(&url)
            .header(
                "Host",
                self.base_url
                    .to_string()
                    .replace("http://", "")
                    .replace("https://", ""),
            )
            .header("Referer", self.base_url.to_string())
            .header(header::ACCEPT, "application/json")
            .send()?;

        let result: QueryResult = response.json().unwrap();
        if result.rows.is_empty() {
            return Err("未找到项目ID".into());
        }
        if result.rows[0].edit_status > 2 {
            return Err("没有权限修改".into());
        }
        Ok(result.rows[0].project_id.clone())
    }

    fn post_file(
        &self,
        project_id: &str,
        file_id: &str,
        file_buffer: Vec<u8>,
        file_name: &str,
        file_type: &str, // 'goodsfile' 或 'batteryfile'
    ) -> Result<(), Box<dyn std::error::Error>> {
        let blob = multipart::Part::bytes(file_buffer).file_name(file_name.to_string());

        let dir = format!("project/{}/{}", project_id, file_type);
        let initial_preview = "[]";
        let initial_preview_config = "[]";
        let initial_preview_thumb_tags = "[]";

        let form = multipart::Form::new()
            .text("file", file_name.to_string())
            .text("fileId", file_id.to_string())
            .text("initialPreview", initial_preview.to_string())
            .text("initialPreviewConfig", initial_preview_config.to_string())
            .text(
                "initialPreviewThumbTags",
                initial_preview_thumb_tags.to_string(),
            )
            .text("dir", dir)
            .text("fileType", file_type.to_string())
            .text("typeId", project_id.to_string())
            .text("refesh", "true")
            .text("allowedFileTypes", "pdf")
            .text("checkpdf", "true")
            .part("file", blob);

        // let url = format!("{}/rest/document/upload", "http://127.0.0.1:3000");
        let url = format!("{}/rest/document/upload", self.base_url);
        let response = self.client.post(url).multipart(form).send()?;

        if response.status().is_success() {
            println!("文件上传成功: {:?}", response.text().unwrap());
            Ok(())
        } else {
            println!("文件上传失败，状态码: {:?}", response.status());
            Err("文件上传失败".into())
        }
    }
}

fn main() {
    dotenv::from_path("local.env").ok();
    let base_url = env::var("BASE_URL")
        .map_err(|err| {
            eprintln!("Error reading BASE_URL: {}", err);
            exit(1);
        })
        .unwrap();

    let username = env::var("USER_NAME")
        .map_err(|err| {
            eprintln!("Error reading USER_NAME: {}", err);
            exit(1);
        })
        .unwrap();
    let password = env::var("PASSWORD")
        .map_err(|err| {
            eprintln!("Error reading PASSWORD: {}", err);
            exit(1);
        })
        .unwrap();
    let mut client = HttpClient::new(base_url, username, password);
    client.login().unwrap();
    let start_path = Path::new(r"Z:\");
    traverse_directory(start_path, 1, &client);
}
