fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        *s = &s[i + delimiter.len_utf8()..];
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: \"{}\", s1: \"{}\", s: \"{}\"", hello, s1, s);

    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ',');
    println!("hello is: \"{}\", s1: \"{}\", s: \"{}\"", hello, s1, s);

    let s = "This a sample string".to_owned();
    let mut s1 = s.as_str();
    while !s1.is_empty() {
        let word = strtok(&mut s1, ' ');
        println!("\"{}\"", word);
    }
}
