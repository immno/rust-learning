use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();

        *started = true;
        eprintln!("I'm a happy worker!");

        cvar.notify_one();
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("working...");
        }
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap()
    }
    eprintln!("Worker started!")

}
