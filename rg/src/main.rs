use clap::{App, Arg};
use colored::*;
use regex::Regex;
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
    let file = matches.value_of("file").unwrap_or_default();
    let f = File::open(file)?;
    let r = BufReader::new(f);
    if matches.is_present("regex") {
        let pattern = matches.value_of("pattern").unwrap();
        let re = Regex::new(pattern)?;
        r.lines().enumerate().for_each(|(no, line)| match line {
            Ok(l) => {
                if let Some(m) = re.find(&l) {
                    println!(
                        "{:0>4}: {}{}{}",
                        no.to_string().blue(),
                        &l[..m.start()],
                        m.as_str().red(),
                        &l[m.end()..],
                    )
                }
            }
            Err(e) => println!("{}: {:?}", no.to_string().blue(), e),
        });
    } else {
        let ref pattern: String = matches.value_of("pattern").unwrap().to_string();
        r.lines().enumerate().for_each(|(no, line)| match line {
            Ok(l) => {
                if let Some(s) = l.find(pattern) {
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
    }

    Ok(())
}
