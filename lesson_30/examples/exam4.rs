fn main() {
    let r1 = 0xdeadbeef as *mut u32;
    println!("so far so good!");

    unsafe {
        *r1 += 1;
        println!("r1: {:?}", *r1);
    }
}