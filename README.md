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

## 1.4 多模块workspace
修改`Cargo.toml`文件包含了：整个工作空间。不会包含 `[package]`
```toml
[workspace]

members = [
    "adder",
]
```
接下来，在项目`根目录`运行`cargo new adder`：
```shell
$ cargo new adder
     Created binary (application) `adder` project
```
到此为止，可以运行`cargo build`来构建工作空间。`根目录`中的文件应该看起来像这样：
```haml
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
接着新生成一个叫做`add-one`的库(lib)：
```shell
$ cargo new add-one --lib
     Created library `add-one` project
```
```haml
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
在`add-one/src/lib.rs`文件中，增加一个`add_one`函数：
文件名: `add-one/src/lib.rs`
```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
现在工作空间中有了一个库`crate`，让`adder`依赖库`crate add-one`。首先需要在`adder/Cargo.toml`文件中增加`add-one`作为路径依赖：
文件名: `adder/Cargo.toml`
```toml
[dependencies]

add-one = { path = "../add-one" }
```
`cargo`并不假定工作空间中的Crates会相互依赖，所以需要明确表明工作空间中 crate 的依赖关系。  
接下来，在`adder crate`中使用`add-one crate`的函数`add_one`打开`adder/src/main.rs`在顶部增加一行`use add-one`;将库`crate`引入作用域。  
接着修改`main`函数来调用`add_one`函数,文件名: `adder/src/main.rs`:  
```rust
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```