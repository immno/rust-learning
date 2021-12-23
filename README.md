# 1 安装
[安装指南](https://www.rust-lang.org/tools/install)  
[向导指南](https://www.rust-lang.org/learn/get-started)  
[国内源](https://mirrors.ustc.edu.cn/help/crates.io-index.html)  
Rust 更新非常频繁。如果您之前安装过 Rustup，则您的 Rust 版本可能已经过时。  
通过运行 `rustup update` 获取最新版本的 Rust。  

## 1.1 Cargo
当您安装 Rustup 时，您还将获得 Rust 构建工具和包管理器的最新稳定版本，也称为 Cargo。  
- 初始化已有文件夹的项目: `cargo init`
- 构建项目: `cargo build`
- 启动项目: `cargo run`
- 测试项目: `cargo test`
- 为项目构建文档: `cargo doc`
- 将库发布到 crates.io: `cargo publish`
- 要测试您是否安装了 Rust 和 Cargo: `cargo --version`

## 1.2 开始新项目
让我们用新的 Rust 开发环境编写一个小应用程序。  
首先，我们将使用 Cargo 为我们创建一个新项目：`cargo new hello-rust`  
如果文件夹存在则使用: `cargo init`

## 1.3 官方demo
在`Cargo.toml`文件中:
```toml
[dependencies]
ferris-says = "0.2"
```
构建项目: `cargo build`
开始写一个小的项目，在`main.rs`文件中:
```rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```
运行这个应用: `cargo run`