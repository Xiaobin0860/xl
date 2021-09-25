use regex::Regex;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug)]
struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(self.buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T: FromStr> Parse for T {
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_owned()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_| "failed to parse captured string".to_owned())
                })
        } else {
            Err("failed to parse string".to_owned())
        }
    }
}

use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Complex {
    a: f64,
    b: f64,
}

impl Complex {
    fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let a = self.a + rhs.a;
        let b = self.b + rhs.b;
        Self::new(a, b)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.a + rhs.a, self.b + rhs.b)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        Complex::new(self.a + rhs, self.b)
    }
}

struct Cat;
struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn name1(animal: &impl Animal) -> &'static str {
    animal.name()
}

fn name2<T: Animal>(animal: &T) -> &'static str {
    animal.name()
}

fn name3<T>(animal: &T) -> &'static str
where
    T: Animal,
{
    animal.name()
}

fn name4(animal: &dyn Animal) -> &'static str {
    animal.name()
}

struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char,
}
impl<'a> SentenceIter<'a> {
    pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
        Self { s, delimiter }
    }
}
impl<'a> Iterator for SentenceIter<'a> {
    type Item = &'a str;
    // 想想 Item 应该是什么类型？
    fn next(&mut self) -> Option<Self::Item> {
        // 如何实现 next 方法让下面的测试通过？
        if let Some(idx) = self.s.find(self.delimiter) {
            let len = self.delimiter.len_utf8();
            let s = &self.s[..idx + len];
            *self.s = &self.s[idx + len..];
            Some(s.trim())
        } else {
            let s = self.s.trim();
            *self.s = "";
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
    }
}

fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"Hello world!").unwrap();
    println!("{:?}", buf);

    let c1 = Complex::new(1.0, 1 as f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 5.0);
    println!("{:?}", c1 + c2);
    println!("{:?}", c1 + c2);

    let cat = Cat;
    println!("cat1: {}", name1(&cat));
    let dog = Dog;
    println!("dog2: {}", name2(&dog));
    println!("dog3: {}", name3(&dog));
    println!("cat4: {}", name4(&cat));
    println!("dog4: {}", name4(&dog));

    let mut s = "a。 b。 c";
    let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
    println!("sentences: {:?}", sentences);

    use std::fmt::{Debug, Display};
    use std::mem::transmute;

    let s1 = String::from("hello world!");
    let s2 = String::from("goodbye world!");
    // Display / Debug trait object for s
    let w1: &dyn Display = &s1;
    let w2: &dyn Debug = &s1;

    // Display / Debug trait object for s1
    let w3: &dyn Display = &s2;
    let w4: &dyn Debug = &s2;

    // 强行把 triat object 转换成两个地址 (usize, usize)
    // 这是不安全的，所以是 unsafe
    let (addr1, vtable1): (usize, usize) = unsafe { transmute(w1) };
    let (addr2, vtable2): (usize, usize) = unsafe { transmute(w2) };
    let (addr3, vtable3): (usize, usize) = unsafe { transmute(w3) };
    let (addr4, vtable4): (usize, usize) = unsafe { transmute(w4) };

    // s 和 s1 在栈上的地址，以及 main 在 TEXT 段的地址
    println!(
        "s1: {:p}, s2: {:p}, main(): {:p}",
        &s1, &s2, main as *const ()
    );
    // trait object(s / Display) 的 ptr 地址和 vtable 地址
    println!("addr1: 0x{:x}, vtable1: 0x{:x}", addr1, vtable1);
    // trait object(s / Debug) 的 ptr 地址和 vtable 地址
    println!("addr2: 0x{:x}, vtable2: 0x{:x}", addr2, vtable2);

    // trait object(s1 / Display) 的 ptr 地址和 vtable 地址
    println!("addr3: 0x{:x}, vtable3: 0x{:x}", addr3, vtable3);

    // trait object(s1 / Display) 的 ptr 地址和 vtable 地址
    println!("addr4: 0x{:x}, vtable4: 0x{:x}", addr4, vtable4);

    // 指向同一个数据的 trait object 其 ptr 地址相同
    assert_eq!(addr1, addr2);
    assert_eq!(addr3, addr4);

    // 指向同一种类型的同一个 trait 的 vtable 地址相同
    // 这里都是 String + Display
    assert_eq!(vtable1, vtable3);
    // 这里都是 String + Debug
    assert_eq!(vtable2, vtable4);
}

#[test]
fn parse_should_work() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert!(u32::parse("123.45abcd").is_err());
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}

#[test]
fn it_works() {
    let mut s = "This is the 1st sentence. This is the 2nd sentence.";
    let mut iter = SentenceIter::new(&mut s, '.');
    assert_eq!(iter.next(), Some("This is the 1st sentence."));
    assert_eq!(iter.next(), Some("This is the 2nd sentence."));
    assert_eq!(iter.next(), None);
}

use std::fs::File;
#[allow(dead_code)]
fn unused_fn() {
    let mut f = File::create("/tmp/test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello ").unwrap();
    // let w1 = w.by_ref();
    // w1.write_all(b"world").unwrap();
}
