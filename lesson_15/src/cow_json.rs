use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct User<'input>{
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

fn main() {
    let input = r#"{ "name": "Mno", "age": 18}"#;
    let user:User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(v) => println!("Borrowed {}", v),
        Cow::Owned(v) => println!("Owned {}", v),
    }

}