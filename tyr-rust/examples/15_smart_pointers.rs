use serde::Deserialize;
use std::borrow::Cow;
use url::Url;

#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

// MyString 里，String 有 3 个 word，共 24 字节，所以它以 8 字节对齐
// 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节。
// MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节.
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(v: impl AsRef<str>) -> Self {
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

// 实现 Deref 接口对两种不同的场景统一得到 &str
impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(v) => v.deref(),
            MyString::Standard(v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        if s.len() > MINI_STRING_MAX_LEN {
            Self::Standard(s.to_string())
        } else {
            Self::Inline(MiniString::new(s))
        }
    }
}

impl From<String> for MyString {
    fn from(s: String) -> Self {
        if s.len() > MINI_STRING_MAX_LEN {
            Self::Standard(s)
        } else {
            Self::Inline(MiniString::new(s))
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    pub fn push_str(&mut self, s: &str) {
        match self {
            MyString::Inline(inls) => {
                let cur_len = inls.len();
                if cur_len + s.len() > MINI_STRING_MAX_LEN {
                    let mut stds = inls.to_string();
                    stds.push_str(s);
                    *self = Self::Standard(stds);
                } else {
                    inls.data[cur_len..].copy_from_slice(s.as_bytes());
                    inls.len += s.len() as u8;
                }
            }
            MyString::Standard(stds) => stds.push_str(s),
        }
    }
}

fn main() {
    let url = Url::parse("http://tyr.com/rust?page=1024&sort=desc&extra=hello%20world>").unwrap();
    let mut pairs = url.query_pairs();
    assert_eq!(pairs.count(), 3);
    let (mut k, v) = pairs.next().unwrap();
    println!("k: {}, v: {}", k, v);
    k.to_mut().push_str("_lala");
    print_pairs((k, v));
    print_pairs(pairs.next().unwrap());
    print_pairs(pairs.next().unwrap());

    let input = r#"{"name": "xl000", "age": 18}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("Borrowed {}", x),
        Cow::Owned(x) => println!("Owned {}", x),
    }

    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));

    let s = String::from("这是一个超过了三十个字节的很长很长的字符串");
    println!("s: {:p}", &*s);
    // From<T: AsRef<str>> 的实现会导致额外的复制
    let s3: MyString = s.into();
    println!("s3: {:p}", &*s3);

    let mut s4: MyString = "Hello Tyr! ".into();
    println!("s4: {:?}", s4);
    s4.push_str("这是一个超过了三十个字节的很长很长的字符串");
    println!("s4: {:?}", s4);
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("k: {}, v: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v),
    }
}
