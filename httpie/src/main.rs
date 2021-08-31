use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
use reqwest::Url;

#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Clap, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv))]
    body: Vec<KV>,
}

#[derive(Debug)]
struct KV {
    k: String,
    v: String,
}

impl FromStr for KV {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv(s: &str) -> Result<KV> {
    Ok(s.parse()?)
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
