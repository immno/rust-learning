use std::{
    sync::{Arc, Mutex},
    thread,
};

fn arc_mutex_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handler = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handler.join().unwrap();
    println!("a= {:?}", a);
}

fn main() {
    arc_mutex_is_send_sync();
}