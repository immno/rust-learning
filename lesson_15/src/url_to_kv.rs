use std::borrow::Cow;

use url::Url;

fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();

    let mut pairs = url.query_pairs();
    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    println!("key: {}, v: {}", k, v);

    k.to_mut().push_str("_lala");
    print_pairs((k, v));
    print_pairs(pairs.next().unwrap());
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v),
    }
}