use clap::{App, Arg};
use rg::{GrepError, Greper};

fn main() -> Result<(), GrepError> {
    let matches = App::new("rgrep")
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .required(true),
            Arg::with_name("file").value_name("FILE").required(true),
        ])
        .get_matches();
    let file = matches.value_of("file").unwrap().to_string();
    let pattern = matches.value_of("pattern").unwrap().to_string();

    let greper = Greper::new(pattern, file);
    greper.match_with_default_strategy()
}
