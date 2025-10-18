# Daemon_Demi
A Rust based daemon file watcher using notify and sysinfo
A lightweight file watcher daemon written in Rust.  
It monitors directory changes using [`notify`](https://crates.io/crates/notify) and integrates system metrics via [`sysinfo`](https://crates.io/crates/sysinfo).

## Features
- Real-time file change detection
- CPU and memory usage tracking
- JSON logging output

## Build & Run
```bash
cargo build --release
./target/release/daemon_demi
