use pest::iterators::Pair;
use rlua::{LuaParser, Parser, Rule};

fn main() {
    let pairs = LuaParser::parse(Rule::Chunk, include_str!("lus/onLuaSocket.lua")).unwrap();
    for pair in pairs {
        visit_string(pair);
    }
    let pairs = LuaParser::parse(Rule::Chunk, include_str!("lus/sLairhunt.lua")).unwrap();
    for pair in pairs {
        visit_string(pair);
    }
    let pairs = LuaParser::parse(Rule::Chunk, include_str!("lus/sLogin.lua")).unwrap();
    for pair in pairs {
        visit_string(pair);
    }
}

fn visit_string(pair: Pair<Rule>) {
    match pair.as_rule() {
        Rule::StringContentSQ | Rule::StringContentDQ | Rule::StringContentLong => {
            let s = pair.as_str();
            if s.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}') {
                println!("{}", s);
            }
        }
        _ => {
            for inner in pair.into_inner() {
                visit_string(inner);
            }
        }
    }
}
