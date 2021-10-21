use clap::{App, Arg};
use colored::*;
use rg::GrepError;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), GrepError> {
    let matches = App::new("rgrep")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("regex")
                .short("e")
                .help("use PATTERN for regex matching"),
        )
        .args(&[
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .required(true),
            Arg::with_name("file").value_name("FILE"),
        ])
        .get_matches();
    println!("{:?}", matches);
    let is_regex = matches.is_present("regex");
    let pattern: String = matches.value_of("pattern").unwrap().to_string();
    let file = matches.value_of("file").unwrap_or_default();
    println!("regex={}, pattern={}, file={}", is_regex, pattern, file);

    let f = File::open(file)?;
    let r = BufReader::new(f);
    r.lines().enumerate().for_each(|(no, line)| match line {
        Ok(l) => {
            if let Some(s) = l.find(pattern.as_str()) {
                let end = s + pattern.len();
                println!(
                    "{:0>4}: {}{}{}",
                    no.to_string().blue(),
                    &l[..s],
                    &l[s..end].red(),
                    &l[end..],
                )
            }
        }
        Err(e) => println!("{}: {:?}", no.to_string().blue(), e),
    });

    Ok(())
}
