// 添加Copy编译报错，String没有实现Copy
#[derive(Debug, Clone)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

// 如果 a 已经存在，在 clone 过程中会分配内存，那么用 a.clone_from(&b) 可以避免内存分配，提高效率。
fn main() {
    let dev = Developer {
        name: "Mno".into(),
        age: 18,
        lang: Language::Rust,
    };
    let dev1 = dev.clone();
    println!("dev: {:?}, addr of dev name: {:p},", dev, dev.name.as_str());
    println!("dev1: {:?}, addr of dev1 name: {:p},", dev1, dev1.name.as_str());
}