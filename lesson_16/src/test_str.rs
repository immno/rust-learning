use std::fmt;

fn main() {
    let s = String::from("hello");
    print_slice(&s);
    print_slice(&s[..]);

    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    print_slice2(&s);
    print_slice2(&s[..]);
    print_slice2(s);
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
    where
        T: AsRef<[U]>,
        U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    #[test]
    fn test() {
        let arr = ['h', 'e', 'l', 'l', 'o'];
        let vec = vec!['h', 'e', 'l', 'l', 'o'];
        let s = String::from("hello");
        let s1 = &arr[1..3];
        let s2 = &vec[1..3];
        let s3 = &s[1..3];

        println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
        assert_eq!(s1, s2);
        assert_eq!(s2, s3.chars().collect::<Vec::<_>>());
        assert_eq!(String::from_iter(s2), s3);
    }
}