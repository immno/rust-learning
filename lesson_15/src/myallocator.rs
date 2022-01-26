use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("ALLOC: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("FREE: {:p}, size {}", ptr, layout.size());
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

fn main() {
    let data = Box::new(Matrix::default());
    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    #[test]
    fn stack_overflow() {
        // 栈溢出 stack overflow
        // 但如果你在本地使用 “cargo run —release” 编译成 release 代码运行，会正常执行！
        let boxed = Box::new([0u8; 1 << 24]);
        println!("len: {}", boxed.len());
    }

    #[test]
    fn borrow() {
        let s = "Hello, world!".to_owned();
        let r1: &String = s.borrow();
        let r2: &str = s.borrow();
        println!("r1: {:p}, r2: {:p}", r1, r2);
    }
}