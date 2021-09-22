use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::net::TcpStream;

struct MyReader<R> {
    reader: R,
    buf: String,
}

impl<R> MyReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

impl<R> MyReader<R>
where
    R: Read,
{
    fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let f = File::open("Cargo.toml").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));
    let size = reader.process().unwrap();
    println!("total size read: {}", size);

    let stream = TcpStream::connect("baidu.com:80").unwrap();
    let mut writer = MyWriter::new(BufWriter::new(stream));
    writer.write("hello world!").unwrap();
}
