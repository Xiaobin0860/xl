use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::size_of;

#[allow(dead_code)]
enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}  {}  {} {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>", "Result<T, ()>"
        );
        println!("{}", "-".repeat(80));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:8} {:12} {:14}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
            size_of::<Result<$t, ()>>()
        );
    };
}

fn main() {
    show_size!(header);
    show_size!(u8);
    show_size!(i128);
    show_size!(f64);
    show_size!(Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
    show_size!(Cow<[u8]>);
    show_size!(Cow<str>);
}
