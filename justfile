set shell := ["powershell.exe", "-c"]

build-c:
    cargo build --bin client --features client

build-s:
    cargo build --bin server --features server

build-cf:
    cargo build --bin cfth --features cfth

build-d:
    cargo build --bin doc --features doc

dev-c:
    cargo run --features client --bin client

dev-s:
    cargo run --features server --bin server

dev-cf:
    cargo run --features cfth --bin cfth

dev-d:
    cargo run --features doc --bin doc

lint-c:
    cargo clippy -- -D warnings --bin client --features client

lint-s:
    cargo clippy -- -D warnings --bin server --features server  

lint-cf:
    cargo clippy -- -D warnings --bin cfth --features cfth

lint-d:
    cargo clippy -- -D warnings --bin doc --features doc

build-r:
    cargo build --bin client --features client --release
    cargo build --bin server --features server --release
    cargo build --bin cfth --features cfth --release
    cargo build --bin doc --features doc --release
