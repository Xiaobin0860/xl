use std::thread;
use std::{collections::HashMap, mem::size_of_val};

fn main() {
    let s = String::from("hello world");
    let handle = thread::spawn(move || {
        println!("moved: {:?}", s);
    });
    handle.join().unwrap();

    // 长度为 0
    let c1 = || println!("hello world!");
    // 和参数无关，长度也为 0
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("tyr");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 如果捕获一个引用，长度为 8
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };
    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    );

    let name = String::from("xl000");
    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());
    //println!("name: {}", name);
    // 所以 c1 可以被调用多次
    println!("c1 call once: {:?}", c("qiao".into()));
    println!("c1 call twice: {:?}", c("bonjour".into()));
    // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
    println!("result: {:?}", call_once("hi".into(), c));
    // 无法再次调用
    // let result = c("hi".to_string());
    // fn 也可以被当成 fn 调用，只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure));

    let mut name = String::from("hello");
    let mut name1 = String::from("hola");
    // 捕获 &mut name
    let mut c = || {
        name.push_str(" xl000");
        println!("c: {}", name);
    };
    // println!("name: {}", name);
    // 捕获 mut name1，注意 name1 需要声明成 mut
    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };
    // println!("name1: {}", name1);
    c();
    c1();
    call_mut(&mut c);
    call_mut(&mut c1);
    call_once2(c);
    call_once2(c1);

    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];
    // Fn，不移动所有权
    let mut c = |x: u64| v.len() as u64 * x;
    // Fn，移动所有权
    let mut c1 = move |x: u64| v1.len() as u64 * x;
    println!("direct call: {}", c(2));
    println!("direct call: {}", c1(2));
    println!("call: {}", call(3, &c));
    println!("call: {}", call(3, &c1));
    println!("call_mut: {}", call_mut3(4, &mut c));
    println!("call_mut: {}", call_mut3(4, &mut c1));
    println!("call_once: {}", call_once3(5, c));
    println!("call_once: {}", call_once3(5, c1));

    let name = String::from("Tyr");
    let vec = vec!["Rust", "Elixir", "Javascript"];
    let v = &vec[..];
    let data = (1, 2, 3, 4);
    let c = move || {
        println!("data: {:?}", data);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };
    // println!("name: {}", name);
    c();
    // println!("name: {}", name);
    println!("c: {}", size_of_val(&c),);

    let env = "PATH=/usr/bin".to_string();
    let cmd = "cat /etc/passwd";
    let r1 = execute(cmd, BashExecutor { env: env.clone() });
    println!("{:?}", r1);
    let r2 = execute(cmd, |cmd: &str| {
        Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
    });
    println!("{:?}", r2);
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}

// 在作为参数时，FnMut 也要显式地使用 mut，或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 想想看，为啥 call_once 不需要 mut？
fn call_once2(c: impl FnOnce()) {
    c();
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

pub trait Executor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str>;
}

struct BashExecutor {
    env: String,
}

impl Executor for BashExecutor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!(
            "fake bash execute: env: {}, cmd: {}",
            self.env, cmd
        ))
    }
}

fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
    exec.execute(cmd)
}

impl<F> Executor for F
where
    F: Fn(&str) -> Result<String, &'static str>,
{
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        self(cmd)
    }
}
