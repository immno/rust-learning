use std::ops::Deref;

fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);
    println!("cap should be 8: {}", v1.capacity());

    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be 5: {}", v2.capacity());

    assert!(b2.deref() == v2);

    b2[0] = 2;
    println!("b2: {:?}", b2);

    let b3 = Box::new([2, 2, 3, 4, 5]);
    println!("b3: {:?}", b3);

    assert!(b2 == b3);
}