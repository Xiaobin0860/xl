use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
use colored::Colorize;
use lazy_static::lazy_static;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "xl000 <l_xb@foxmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
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

#[derive(Debug, PartialEq)]
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

lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust httpie".parse()?);
    let client = Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    Ok(print_res(res).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for KV { k, v } in args.body.iter() {
        body.insert(k, v);
    }
    let res = client.post(&args.url).json(&body).send().await?;
    Ok(print_res(res).await?)
}

fn print_status(res: &Response) {
    let status = format!("{:?} {}", res.version(), res.status()).blue();
    println!("{}\n", status)
}

fn print_headers(res: &Response) {
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), value)
    }
    print!("\n");
}

fn get_mime(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => print_syntax(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntax(body, "html"),
        Some(v) if v == mime::TEXT_XML => print_syntax(body, "xml"),
        _ => println!("{}", body),
    }
}

fn print_syntax(body: &String, ext: &str) {
    let syntax = SYNTAX_SET.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &THEME_SET.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(body) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &SYNTAX_SET);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

async fn print_res(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);
    let mime = get_mime(&res);
    let body = res.text().await?;
    print_body(mime, &body);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
        assert_eq!(
            parse_url("https://httpbin.org/post").unwrap(),
            "https://httpbin.org/post"
        );
    }

    #[test]
    fn parse_kv_works() {
        assert!(parse_kv("a").is_err());
        assert!(parse_kv("a=").is_ok());
        assert_eq!(
            parse_kv("a=1").unwrap(),
            KV {
                k: "a".into(),
                v: "1".to_string(),
            }
        );
        assert_eq!(
            parse_kv("a=").unwrap(),
            KV {
                k: "a".into(),
                v: "".to_owned(),
            }
        );
    }
}
