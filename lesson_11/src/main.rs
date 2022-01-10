use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::mem::{align_of, size_of};

// example 1
struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

// example 2
enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

// 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!( "{:<24} {:>4} {} {}",
        "Type", "T", "Option", "Result" );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!( "{:<24} {:4} {:8} {:12}",
        stringify!($t),
        size_of::<$t>(),
        size_of::<Option<$t>>(),
        size_of::<Result<$t, std::io::Error>>(),
        )
    };
}

// example 3
fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"hello mno")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn example_1() {
        println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
        println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
    }

    #[test]
    fn example_2() {
        show_size!(header);
        show_size!(u8);
        show_size!(f64);
        show_size!(&u8);
        show_size!(Box<u8>);
        show_size!(&[u8]);
        show_size!(String);
        show_size!(Vec<u8>);
        show_size!(std::io::Error);
        show_size!(HashMap<String,String>);
        show_size!(E);
        show_size!(Result<String,()>);
    }

}