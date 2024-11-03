set shell := ["powershell.exe", "-c"]

build-c:
    cargo build --bin client --features client

build-s:
    cargo build --bin server --features server

dev-c:
    cargo run --features client --bin client

dev-s:
    cargo run --features server --bin server

lint-c:
    cargo clippy -- -D warnings --bin client --features client

lint-s:
    cargo clippy -- -D warnings --bin server --features server  

build-release:
    cargo build --bin client --features client --release
    cargo build --bin server --features server --release
