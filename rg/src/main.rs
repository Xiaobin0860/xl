use clap::{App, Arg};
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
    let pattern = matches.value_of("pattern").unwrap();
    let file = matches.value_of("file").unwrap_or_default();
    println!("regex={}, pattern={}, file={}", is_regex, pattern, file);

    let f = File::open(file)?;
    let r = BufReader::new(f);
    r.lines()
        .enumerate()
        .for_each(|(no, line)| println!("{}: {}", no, line.unwrap()));

    Ok(())
}
