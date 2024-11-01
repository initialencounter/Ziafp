#[cfg(windows)]
fn main() {
    #[cfg(feature = "client")]
    {
        println!("cargo:warning=正在构建客户端");
        let mut res = winres::WindowsResource::new();
        res.set_icon("./resources/favicon.ico");
        res.compile().unwrap();
        embed_resource::compile("resources/icon.rc", embed_resource::NONE);
    }
    #[cfg(feature = "server")]
    {
        println!("cargo:warning=正在构建服务端");
    }
}

#[cfg(unix)]
fn main() {}

