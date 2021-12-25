mod abi;
mod error;

use std::error::Error;
use std::fs;

// example_0:第一个实用的 Rust 程序
// example_7：错误处理
fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "target/lesson_3_rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}

// example_1:变量和函数

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

// example_2

fn pi() -> f64 {
    std::f64::consts::PI
}

fn not_pi() {
    std::f64::consts::PI;
}

// example_3:数据结构
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

// example_4:控制流程

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += i;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}
// example_5：在执行过程中，IntoIterator 会生成一个迭代器

// example_6：模式匹配
fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::{apply, cube, Event, fib_for, fib_loop, fib_while, Gender, not_pi, pi, process_event, process_message, square, Topic, TopicId, User, UserId};

    #[test]
    fn example_1() {
        println!("apply square: {}", apply(2, square));
        println!("apply cube: {}", apply(2, cube));
    }

    #[test]
    fn example_2() {
        let is_pi = pi();
        let is_unit1 = not_pi();
        let is_unit2 = { pi() };

        println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2)
    }

    #[test]
    fn example_3() {
        let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
        let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

        let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
        let event1 = Event::Join((alice.id, topic.id));
        let event2 = Event::Join((bob.id, topic.id));
        let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

        println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
    }

    #[test]
    fn example_4() {
        let n = 10;
        fib_loop(n);
        fib_while(n);
        fib_for(n);
    }

    #[test]
    fn example_5() {
        let arr = [1, 2, 3];
        assert_eq!(arr[..], [1, 2, 3]);
        assert_eq!(arr[0..1], [1]);
        assert_eq!(arr[0..=1], [1, 2]);
    }

    #[test]
    fn example_6() {
        let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
        let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

        let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
        let event1 = Event::Join((alice.id, topic.id));
        let event2 = Event::Join((bob.id, topic.id));
        let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

        process_event(&event1);
        process_event(&event2);
        process_event(&event3);
    }
}