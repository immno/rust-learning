use std::mem::transmute;

fn main() {
    let data = unsafe {
        let p = libc::malloc(8);
        let arr: &mut [u8; 8] = transmute(p);
        arr
    };
    data.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);

    println!("data: {:?}", data);

    unsafe {
        libc::free(transmute(data));
    }
}