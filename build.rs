#[cfg(windows)]
fn main() {
    #[cfg(feature = "client")]
    {
        println!("cargo:warning=正在构建客户端");
        let mut res = winres::WindowsResource::new();
        res.set_icon("./resources/favicon.ico");
        res.set_manifest(
            r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator" />
                </requestedPrivileges>
            </security>
        </trustInfo>
        </assembly>
        "#,
        );
        res.compile().unwrap();
    }
    #[cfg(feature = "server")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./resources/favicon.ico");
        res.compile().unwrap();
        println!("cargo:warning=正在构建服务端");
    }
}

#[cfg(unix)]
fn main() {}
