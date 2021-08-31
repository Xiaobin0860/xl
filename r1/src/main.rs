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
