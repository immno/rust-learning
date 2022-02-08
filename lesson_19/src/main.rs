use std::{collections::HashMap, mem::size_of_val};

fn main() {
    let c1 = || println!("hello world");

    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("Mno");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    let c3 = || println!("hello: {}", name);
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!("c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
             size_of_val(&c1),
             size_of_val(&c2),
             size_of_val(&c3),
             size_of_val(&c4),
             size_of_val(&c5),
             size_of_val(&main),
    );
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use std::process::Output;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn fn_once1() {
        let name = String::from("Mno");
        let c = move |greeting: String| (greeting, name);

        let result = c("hello".to_string());

        println!("result: {:?}", result);
        // 无法再次调用
        // let result = c("hi".to_string());
    }

    fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
        c(arg)
    }

    fn not_closure(arg: String) -> (String, String) {
        (arg, "Rosie".into())
    }

    #[test]
    fn fn_once2() {
        let name = String::from("Mno");
        let c = move |greeting: String| (greeting, name.clone());

        println!("c1 call once: {:?}", c("qiao".into()));
        println!("c1 call twice: {:?}", c("bonjour".into()));

        println!("result: {:?}", call_once("hi".into(), c));

        // 无法再次调用
        // let result = c("hi".to_string());

        println!("result: {:?}", call_once("hola".into(), not_closure))
    }

    fn call_mut(c: &mut impl FnMut()) {
        c();
    }

    fn call_once2(c: impl FnOnce()) {
        c();
    }

    #[test]
    fn fn_mut1() {
        let mut name = String::from("hello");
        let mut name1 = String::from("hola");

        let mut c = || {
            name.push_str(" Mno");
            println!("c: {}", name);
        };

        let mut c1 = move || {
            name1.push_str("!");
            println!("c1: {}", name1);
        };

        c();
        c1();

        call_mut(&mut c);
        call_mut(&mut c1);

        call_once2(c);
        call_once2(c1);
    }

    fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
        c(arg)
    }

    fn call_mut3(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
        c(arg)
    }

    fn call_once3(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
        c(arg)
    }

    #[test]
    fn fn1() {
        let v = vec![0u8; 1024];
        let v1 = vec![0u8; 1023];

        let mut c = |x: u64| v.len() as u64 * x;
        let mut c1 = move |x: u64| v1.len() as u64 * x;

        println!("direct call: {}", c(2));
        println!("direct call: {}", c1(2));

        println!("call: {}", call(3, &c));
        println!("call: {}", call(3, &c1));

        println!("call_mut: {}", call_mut3(4, &mut c));
        println!("call_mut: {}", call_mut3(4, &mut c1));

        println!("call_once: {}", call_once3(5, c));
        println!("call_once: {}", call_once3(5, c1));
    }

    fn curry<T>(x: T) -> impl Fn(T) -> T
        where
            T: Mul<Output=T> + Copy,
    {
        move |y| x * y
    }

    #[test]
    fn example1() {
        let c1 = curry(5);
        println!("5 multiply 2 is: {}", c1(2));

        let adder2 = curry(3.14);
        println!("pi multiply 4^2 is: {}", adder2(4. * 4.));
    }

    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    #[test]
    fn example2() {
        let intensity: u32 = 2;
        let random_number: u32 = 3;
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_result.value(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    #[test]
    fn homework_1() {
        let name = String::from("Tyr");
        let vec = vec!["Rust", "Elixir", "Javascript"];
        let v = &vec[..];
        let data = (1, 2, 3, 4);
        let c = move || {
            println!("data: {:?}", data);
            println!("v: {:?}, name: {:?}", v, name.clone());
        };
        c();

        // 请问在这里，还能访问 name 么？为什么？
        // 不能：move
    }

    #[test]
    fn homework_2() {}

    #[test]
    fn homework_3() {
        let env = "PATH=/usr/bin".to_string();

        let cmd = "cat /etc/passwd";
        let r1 = execute(cmd, BashExecutor { env: env.clone() });
        println!("{:?}", r1);

        // 看看我给的 tonic 的例子，想想怎么实现让 27 行可以正常执行
        let r2 = execute(cmd, |cmd: &str| {
            Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
        });
        println!("{:?}", r2);
    }


    pub trait Executor {
        fn execute(&self, cmd: &str) -> Result<String, &'static str>;
    }

    pub struct BashExecutor {
        env: String,
    }

    impl Executor for BashExecutor {
        fn execute(&self, cmd: &str) -> Result<String, &'static str> {
            Ok(format!("fake bash execute: env: {}, cmd: {}",
                       self.env, cmd
            ))
        }
    }

    impl<F> Executor for F
        where
            F: Fn(&str) -> Result<String, &'static str>,
    {
        fn execute(&self, cmd: &str) -> Result<String, &'static str> {
            self(cmd)
        }
    }

    pub fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
        exec.execute(cmd)
    }
}
