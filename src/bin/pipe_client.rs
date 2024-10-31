use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("正在连接到管道...");
    
    // 连接到命名管道
    let mut pipe = OpenOptions::new()
        .read(true)
        .write(true)
        .open(r"\\.\pipe\pdf_processor")?;

    // 发送消息
    let message = "你好，这是来自客户端的消息！";
    pipe.write_all(message.as_bytes())?;
    pipe.flush()?;

    println!("消息已发送: {}", message);
    Ok(())
}
// cargo run --bin pipe_client