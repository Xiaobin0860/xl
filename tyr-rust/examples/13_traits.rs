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

#[test]
fn parse_should_work() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert!(u32::parse("123.45abcd").is_err());
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}

fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"Hello world!").unwrap();
    println!("{:?}", buf);
}
