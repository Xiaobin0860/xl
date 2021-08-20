//! # Craft A Language 01
//!
//! 本节目的是迅速实现一个最精简的语言功能，了解一门计算机语言的骨架。
//!
//! ## 知识点：
//!
//! 1. 递归下降法做词法分析
//! 2. 语义分析中的引用消解
//! 3. 通过遍历AST执行程序
//!
//! ### 语法规则
//!
//! 本节采用的语法规则是极其精简的, 只能定义函数和调用函数. 定义函数时, 不能有参数.
//! ```EBNF
//! prog = (fnDecl | fnCall)* ;
//! fnDecl : "fn" Identifier '(' ')' fnBody ;
//! fnBody : '{' fnCall* '}';
//! fnCall : Identifier '(' paramList? ')' ;
//! paramList : StringLiteral (',' StringLiteral)* ;
//! ```

#[derive(Debug, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    StringLiteral,
    Seperator,
    Operator,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub text: &'static str,
}

impl Token {
    pub fn new(kind: TokenType, text: &'static str) -> Self {
        Self { kind, text }
    }
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }

    pub fn next(&mut self) -> &Token {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        &self.tokens[self.pos]
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn trace_back(&mut self, new_pos: usize) {
        self.pos = new_pos;
    }
}

fn main() {
    // 一个Token数组，代表了下面这段程序做完词法分析后的结果：
    /*
    //一个函数的声明，这个函数很简单，只打印"Hello World!"
    fn say_hello() {
        println!("Hello World!");
    }
    //调用刚才声明的函数
    say_hello();
    */
    let tokens = [
        Token::new(TokenType::Keyword, "fn"),
        Token::new(TokenType::Identifier, "say_hello"),
        Token::new(TokenType::Seperator, "("),
        Token::new(TokenType::Seperator, ")"),
        Token::new(TokenType::Seperator, "{"),
        Token::new(TokenType::Identifier, "println!"),
        Token::new(TokenType::Seperator, "("),
        Token::new(TokenType::StringLiteral, "Hello World!"),
        Token::new(TokenType::Seperator, ")"),
        Token::new(TokenType::Seperator, ";"),
        Token::new(TokenType::Seperator, "}"),
        Token::new(TokenType::Identifier, "say_hello"),
        Token::new(TokenType::Seperator, "("),
        Token::new(TokenType::Seperator, ")"),
        Token::new(TokenType::Seperator, ";"),
        Token::new(TokenType::Seperator, ";"),
        Token::new(TokenType::EOF, ""),
    ];

    let mut tokenizer = Tokenizer::new(tokens.to_vec());
    loop {
        let t = tokenizer.next();
        println!("{:?}", t);
        match t.kind {
            TokenType::EOF => break,
            _ => continue,
        }
    }
}
