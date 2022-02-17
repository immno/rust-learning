use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex, thread};

lazy_static!{
static ref STORE: Mutex<HashMap<&'static str, &'static [u8]>> = Mutex::new(HashMap::new());
}

fn main() {
    let t1 = thread::spawn(move || {
        let mut store = STORE.lock().unwrap();
        store.insert("hello", b"world");
    });

    let t2 = thread::spawn(move || {
        let mut store = STORE.lock().unwrap();
        store.insert("goodbye", b"world");
    });

    t2.join().unwrap();
    t1.join().unwrap();
    println!("store: {:?}", STORE.lock().unwrap());
}