use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::header;
use reqwest::{multipart, Client};
use warp::Filter;

use ziafp::logger::Logger;
use ziafp::utils::{get_today_date, match_file, popup_message, prepare_file_info, RawFileInfo};

async fn post_file_from_directory(path: PathBuf, client: &HttpClient) -> Vec<String> {
    let raw_file_info = match_file(&path);
    let message = build_confirmation_message(&raw_file_info);

    if !popup_message("警告", &message) {
        return Vec::new();
    }

    let mut uploaded_files = Vec::new();
    for file_info in raw_file_info {
        let result = process_single_file(file_info, client).await;
        if let Ok(file_name) = result {
            uploaded_files.push(file_name);
        }
    }

    uploaded_files
}

fn build_confirmation_message(raw_file_info: &[RawFileInfo]) -> String {
    let mut message = String::from("是否要上传这些文件?：\n");
    for (index, file) in raw_file_info.iter().enumerate() {
        message.push_str(&format!("{}. {}\n", index + 1, file.file_name));
    }
    message
}

async fn process_single_file(file_info: RawFileInfo, client: &HttpClient) -> Result<String> {
    let Some(file_info) = prepare_file_info(file_info) else {
        return Err("准备文件信息失败".into());
    };

    let project_id: String;
    if client.debug {
        project_id = "123456AAAAAAAAAAAAAAAA".to_string();
    } else {
        project_id = client.get_project_id(&file_info.project_no).await?;
    }

    client
        .post_file(
            &project_id,
            &file_info.file_id,
            file_info.file_buffer,
            &file_info.file_name,
            &file_info.file_type,
        )
        .await
}

fn parse_date(date_text: &str) -> Result<(String, String)> {
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
    edit_status: i8,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct DirectoryInfo {
    dir: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct HttpClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
    debug: bool,
    logger: Arc<Mutex<Logger>>,
}

impl HttpClient {
    fn new(
        base_url: String,
        username: String,
        password: String,
        debug: bool,
        log_enabled: bool,
    ) -> Self {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let logger = Arc::new(Mutex::new(Logger::new(
            PathBuf::from("logs"),
            "server",
            log_enabled,
        )));

        HttpClient {
            client,
            base_url,
            username,
            password,
            debug,
            logger,
        }
    }
    async fn heartbeat(&self) -> Result<()> {
        let today_date = get_today_date();
        let _ = self
            .query_project(&format!(
                "systemId=sek&startDate={}&endDate={}&page=1&rows=10",
                today_date, today_date
            ))
            .await?;
        Ok(())
    }
    async fn login(&self) -> Result<()> {
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
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err("登录失败".into())
        }
    }

    async fn query_project(&self, query_string: &str) -> Result<QueryResult> {
        let url = format!("{}/rest/inspect/query?{}", self.base_url, query_string);
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
            .send()
            .await?;

        // 检查是否是401错误
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            // 重新登录
            self.login().await.unwrap();
            // 重试请求
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
                .send()
                .await?;
            let result: QueryResult = response.json().await?;
            return Ok(result);
        }

        let result: QueryResult = response.json().await?;
        Ok(result)
    }
    async fn get_project_id(&self, project_no: &str) -> Result<String> {
        let (start_date, end_date) = parse_date(project_no)?;
        let system_id = if project_no.starts_with("PEK") {
            "pek"
        } else {
            "sek"
        };

        let query_string = format!(
            "systemId={}&category=battery&projectNo={}&startDate={}&endDate={}&page=1&rows=10",
            system_id, project_no, start_date, end_date
        );
        let result: QueryResult = self.query_project(&query_string).await.unwrap();
        if result.rows.is_empty() {
            return Err("未找到项目ID".into());
        }
        if result.rows[0].edit_status > 2 {
            return Err("没有权限修改".into());
        }
        Ok(result.rows[0].project_id.clone())
    }

    async fn post_file(
        &self,
        project_id: &str,
        file_id: &str,
        file_buffer: Vec<u8>,
        file_name: &str,
        file_type: &str, // 'goodsfile' 或 'batteryfile'
    ) -> Result<String> {
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

        let url: String;
        if self.debug {
            url = format!("{}/rest/document/upload", "http://127.0.0.1:3000");
        } else {
            url = format!("{}/rest/document/upload", self.base_url);
        }
        let response = self.client.post(url).multipart(form).send().await?;

        if response.status().is_success() {
            self.logger.lock().await.log(
                "INFO",
                &format!("文件上传成功: {:?}", response.text().await?),
            );
            Ok(file_name.to_string())
        } else {
            self.logger.lock().await.log(
                "ERROR",
                &format!("文件上传失败，状态码: {:?}", response.status()),
            );
            Err("文件上传失败".into())
        }
    }

    async fn log(&self, level: &str, message: &str) {
        self.logger.lock().await.log(level, message);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path("local.env").ok();
    let base_url = env::var("BASE_URL").expect("Error reading BASE_URL");
    let username = env::var("USER_NAME").expect("Error reading USER_NAME");
    let password = env::var("PASSWORD").expect("Error reading PASSWORD");
    let port = env::var("PORT").unwrap_or_else(|_| "25455".to_string());
    let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string());
    let log_enabled = env::var("LOG_ENABLED").unwrap_or_else(|_| "false".to_string());

    let client = Arc::new(Mutex::new(HttpClient::new(
        base_url,
        username,
        password,
        debug == "true",
        log_enabled == "true",
    )));

    if debug == "false" {
        if let Err(e) = client.lock().await.login().await {
            client
                .lock()
                .await
                .logger
                .lock()
                .await
                .log("ERROR", &format!("登录失败: {}", e));
            return Ok(());
        }
    }

    let client_clone = client.clone();
    let client_clone2 = client.clone();
    let heartbeat = tokio::spawn(async move {
        loop {
            if debug == "false" {
                if let Err(e) = client_clone.lock().await.heartbeat().await {
                    client_clone
                        .lock()
                        .await
                        .logger
                        .lock()
                        .await
                        .log("ERROR", &format!("心跳失败: {}", e));
                }
            }
            client_clone
                .lock()
                .await
                .logger
                .lock()
                .await
                .log("INFO", "心跳完成");
            tokio::time::sleep(std::time::Duration::from_secs(60 * 28)).await;
        }
    });
    // 设置 webhook 路由
    let routes = warp::post()
        .and(warp::path("upload"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(warp::any().map(move || client_clone2.clone()))
        .then(
            move |dir: DirectoryInfo, client: Arc<Mutex<HttpClient>>| async move {
                tokio::spawn(async move {
                    let client_guard = client.lock().await;
                    let uploaded_files =
                        post_file_from_directory(PathBuf::from(&dir.dir), &client_guard).await;
                    client_guard
                        .log("INFO", &format!("上传的文件: {:?}", uploaded_files))
                        .await;
                });
                warp::reply::json(&"已接收上传请求")
            },
        );

    // 启动服务器
    warp::serve(routes)
        .run(([0, 0, 0, 0], port.parse::<u16>().unwrap()))
        .await;

    // 等待中断信号
    tokio::signal::ctrl_c().await?;
    println!("服务已关闭");
    heartbeat.abort();
    println!("心跳已关闭");

    Ok(())
}
