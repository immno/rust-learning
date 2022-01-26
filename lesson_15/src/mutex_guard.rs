use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;

lazy_static! {
    static ref METERCS: Mutex<HashMap<Cow<'static, str>, usize>> = Mutex::new(HashMap::new());
}

fn main() {
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            let data = &mut *g;
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
        });
    }

    thread::sleep(Duration::from_millis(100));

    println!("metrics: {:?}", metrics.lock().unwrap());
}