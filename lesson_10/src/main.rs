fn main() {
    println!("Hello, world!");
}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

// fn max(s1: &str, s2: &str) -> &str {
//     if s1 > s2 {
//         s1
//     } else {
//         s2
//     }
// }

// 添加生命周期,使编译通过
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

// 编译器提示给在这，不对,也就是思考题部分
// pub fn strtok<'a>(s: &'a mut & str, delimiter: char) -> &'a str {
pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

struct Employe<'a, 'b> {
    name: &'a str,
    title: &'b str,
    age: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s1 = String::from("Lindsey");
        let s2 = String::from("Rosie");

        let result = max(&s1, &s2);
    }

    #[test]
    fn example_2() {
        let s1 = String::from("Lindsey");
        let s2 = String::from("Rosie");

        let result = max(&s1, &s2);

        println!("bigger one: {}", result);

        let result = get_max(&s1);
        println!("bigger one: {}", result);
    }

    #[test]
    fn example_3() {
        let s = "hello world".to_owned();
        let mut s1 = s.as_str();
        let hello = strtok(&mut s1, ' ');
        println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
    }
}