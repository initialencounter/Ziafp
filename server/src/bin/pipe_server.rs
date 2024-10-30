use std::fs::File;
use std::io::Read;
use std::os::windows::io::FromRawHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::namedpipeapi::ConnectNamedPipe; // 从这里导入
use winapi::um::winbase::{
    CreateNamedPipeA, PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE,
    PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};

fn create_server_pipe() -> std::io::Result<File> {
    unsafe {
        let pipe_name = b"\\\\.\\pipe\\pdf_processor\0";
        let handle = CreateNamedPipeA(
            pipe_name.as_ptr() as *const i8,
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            PIPE_UNLIMITED_INSTANCES,
            1024,
            1024,
            0,
            std::ptr::null_mut(),
        );

        if handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        // 等待客户端连接
        if ConnectNamedPipe(handle, std::ptr::null_mut()) == 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(File::from_raw_handle(handle as *mut std::ffi::c_void))
    }
}

fn main() -> std::io::Result<()> {
    println!("创建命名管道并等待客户端连接...");

    loop {
        let mut pipe = create_server_pipe()?;
        println!("等待客户端连接...");

        let mut buffer = [0; 1024];
        let bytes_read = pipe.read(&mut buffer)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);

        println!("收到消息: {}", message);
    }
}
// cargo run --bin pipe_server
// winapi = { version = "0.3.9", features = ["winbase", "handleapi", "fileapi", "namedpipeapi"] }
