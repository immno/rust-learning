use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// static mut COUNTER: usize = 1;
static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn main() {
    // let t1 = thread::spawn(move || {
    //     unsafe { COUNTER += 10 };
    // });
    // let t2 = thread::spawn(move || {
    //     unsafe { COUNTER += 10 };
    // });
    //
    // t2.join().unwrap();
    // t1.join().unwrap();
    // unsafe { println!("COUNTER: {}", COUNTER) };

    let t1 = thread::spawn(move || {
        COUNTER.fetch_add(10, Ordering::SeqCst);
    });

    let t2 = thread::spawn(move || {
        COUNTER.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| Some(v * 10)).unwrap();
    });

    t2.join().unwrap();
    t1.join().unwrap();
    println!("COUNTER: {}", COUNTER.load(Ordering::Relaxed));
}