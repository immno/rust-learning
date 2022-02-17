fn main() {
    let nonsense = Nonsense;
    nonsense.foo();
    unsafe { nonsense.bar(); }
}

unsafe trait Foo {
    fn foo(&self);
}

trait Bar {
    unsafe fn bar(&self);
}

struct Nonsense;

unsafe impl Foo for Nonsense {
    fn foo(&self) {
        println!("foo!");
    }
}

impl Bar for Nonsense {
    unsafe fn bar(&self) {
        println!("bar!");
    }
}