//fn strtok<'a, 'b>(s: &mut &'a str, delimiters: &'b str) -> &'a str {
fn strtok<'a>(s: &mut &'a str, delimiters: &str) -> &'a str {
    let mut idx = usize::MAX;
    let mut delimiter = ' ';
    for c in delimiters.chars() {
        if let Some(i) = s.find(c) {
            if i < idx {
                idx = i;
                delimiter = c;
            }
        }
    }
    if idx == 0 {
        *s = &s[delimiter.len_utf8()..];
        return strtok(s, delimiters);
    }
    if idx < s.len() {
        let prefix = &s[..idx];
        *s = &s[idx + delimiter.len_utf8()..];
        return prefix;
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, " ");
    println!("hello is: \"{}\", s1: \"{}\", s: \"{}\"", hello, s1, s);

    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ",");
    println!("hello is: \"{}\", s1: \"{}\", s: \"{}\"", hello, s1, s);

    let s = "-This, a sample string.".to_owned();
    let mut s1 = s.as_str();
    while !s1.is_empty() {
        let word = strtok(&mut s1, " ,.-");
        println!("\"{}\"", word);
    }
}
