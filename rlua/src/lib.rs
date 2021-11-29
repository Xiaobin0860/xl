pub use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "lua.pest"]
pub struct LuaParser;
