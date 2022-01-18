fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::net::SocketAddr;

    use super::*;

    #[test]
    fn example_1() {
        let mut map = BTreeMap::new();
        // 去掉insert就会编译报错
        map.insert("hello", "world");
        println!("map: {:?}", map);
    }

    #[test]
    fn example_2() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // let event_numbers = numbers
        let event_numbers: Vec<_> = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect();

        println!("{:?}", event_numbers);
    }

    #[test]
    fn example_3() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let event_numbers = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect::<Vec<_>>();

        println!("{:?}", event_numbers);
    }

    #[test]
    fn example_4() {
        let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
        println!("addr: {:?}, port: {:?}", addr.ip(), addr.port());
    }

    const PI: f64 = 3.141592653589793;
    static E: f32 = 2.71828;

    #[test]
    fn example_5() {
        const V: u32 = 10;
        static V1: &str = "hello";
        println!("PI:{}, E: {}, V: {}, V1: {}", PI, E, V, V1);
    }

    // example_5
    pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
    {
        Borrowed(&'a B),
        Owned(<B as ToOwned>::Owned),
    }
}