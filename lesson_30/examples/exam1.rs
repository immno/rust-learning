use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

fn main() {
    let v = Evil::default();
    let v1 = v.clone();
    let v2 = v.clone();

    let t1 = thread::spawn(move || {
        let v3 = v.clone();
        let mut data = v3.data.borrow_mut();
        *data += 1;
        println!("v3: {:?}", data);
    });

    let t2 = thread::spawn(move || {
        let v4 = v1.clone();
        let mut data = v4.data.borrow_mut();
        *data += 1;
        println!("v4: {:?}", data);
    });

    t2.join().unwrap();
    t1.join().unwrap();

    let mut data = v2.data.borrow_mut();
    *data += 1;
    println!("v2: {:?}", data);
}

#[derive(Debug, Default, Clone)]
struct Evil {
    data: Rc<RefCell<usize>>,
}

unsafe impl Send for Evil {}

