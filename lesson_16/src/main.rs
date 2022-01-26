fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    let s1 = &arr[..2];
    let s2 = &vec[..2];
    let s3 = &vec;
    println!("s1: {:?}, s2: {:?}", s1, s2);

    assert_eq!(s1, s2);
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);
}
