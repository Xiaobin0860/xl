use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("url2md")
        .arg(
            Arg::with_name("url")
                .long("url")
                .help("url to request")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .long("out")
                .help("output file to write to")
                .takes_value(true),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let output = matches.value_of("out").unwrap();

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown: {}", body);
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved to {}", output);
}

pub fn fib_loop(n: u8) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;
    loop {
        if i >= n {
            break;
        }

        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }

    b
}

pub fn fib_while(n: u8) -> i32 {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

pub fn fib_for(n: u8) -> i32 {
    let (mut a, mut b) = (1, 1);
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, fib_loop(1));
        assert_eq!(1, fib_loop(2));
        assert_eq!(2, fib_loop(3));
        assert_eq!(55, fib_loop(10));
        assert_eq!(1, fib_while(1));
        assert_eq!(1, fib_while(2));
        assert_eq!(2, fib_while(3));
        assert_eq!(55, fib_while(10));
        assert_eq!(1, fib_for(1));
        assert_eq!(1, fib_for(2));
        assert_eq!(2, fib_for(3));
        assert_eq!(55, fib_for(10));
    }
}
