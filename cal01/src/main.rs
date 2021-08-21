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

#[derive(Debug, Clone, PartialEq)]
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

///
/// 简化的词法分析器
/// 语法分析器从这里获取Token
///
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
        &self.tokens[self.pos - 1]
    }

    pub fn token(&self, offset: usize) -> &Token {
        let mut pos = self.pos + offset;
        if pos >= self.tokens.len() {
            pos = self.tokens.len() - 1;
        }
        &self.tokens[pos]
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, new_pos: usize) {
        self.pos = new_pos;
    }

    pub fn forwards(&mut self, offset: usize) {
        self.pos += offset;
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    FnBody {
        stmts: Option<Vec<Statement>>,
    },
    FnDecl {
        name: &'static str,
        body: Box<Statement>,
    },
    FnCall {
        name: &'static str,
        params: Option<Vec<&'static str>>,
        decl: Option<Box<Statement>>,
    },
}

impl Statement {
    pub fn fn_body(stmts: Vec<Statement>) -> Self {
        if stmts.is_empty() {
            Self::FnBody { stmts: None }
        } else {
            Self::FnBody { stmts: Some(stmts) }
        }
    }
    pub fn fn_decl(name: &'static str, body: Statement) -> Self {
        Self::FnDecl {
            name,
            body: Box::new(body),
        }
    }
    pub fn fn_call(name: &'static str, params: Vec<&'static str>) -> Self {
        if params.is_empty() {
            Self::FnCall {
                name,
                params: None,
                decl: None,
            }
        } else {
            Self::FnCall {
                name,
                params: Some(params),
                decl: None,
            }
        }
    }
}

pub struct Prog {
    stmts: Vec<Statement>,
}

impl Prog {
    fn new(stmts: Vec<Statement>) -> Self {
        Self { stmts }
    }

    fn dump(&self) {
        for stmt in self.stmts.iter() {
            println!("{:?}", stmt);
        }
    }
}

///
/// 语法分析
/// 包括了AST的数据结构和递归下降的语法解析程序
///
pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    fn next(&mut self) -> &Token {
        self.tokenizer.next()
    }

    fn token(&self, offset: usize) -> &Token {
        self.tokenizer.token(offset)
    }

    fn position(&self) -> usize {
        self.tokenizer.position()
    }

    fn trace_back(&mut self, new_pos: usize) {
        self.tokenizer.set_position(new_pos);
    }

    fn forwards(&mut self, offset: usize) {
        self.tokenizer.forwards(offset);
    }

    pub fn parse_prog(&mut self) -> Prog {
        let mut stmts: Vec<Statement> = Vec::new();
        loop {
            if let Some(stmt) = self.parse_fn_decl() {
                stmts.push(stmt);
                continue;
            }
            if let Some(stmt) = self.parse_fn_call() {
                stmts.push(stmt);
                continue;
            }
            break;
        }
        Prog::new(stmts)
    }

    pub fn parse_fn_body(&mut self) -> Option<Statement> {
        let old_pos = self.position();
        let t = self.next();
        let mut stmts: Vec<Statement> = Vec::new();
        if t.text == "{" {
            while let Some(stmt) = self.parse_fn_call() {
                stmts.push(stmt);
            }
            let t = self.next();
            if t.text == "}" {
                return Some(Statement::fn_body(stmts));
            } else {
                panic!("Expecting '}}' in FunctionBody, while we got a {}", t.text);
            }
        } else {
            println!("Expecting '{{' in FunctionBody, while we got a {}", t.text);
        }
        //如果解析不成功，回溯
        self.trace_back(old_pos);
        None
    }

    ///
    /// 解析函数声明
    /// ```EBNF
    /// fnDecl: "fn" Identifier "(" ")"  fnBody;
    /// ```
    pub fn parse_fn_decl(&mut self) -> Option<Statement> {
        let old_pos = self.position();
        let t = self.next();
        if t.kind == TokenType::Keyword && t.text == "fn" {
            let t = self.next();
            if t.kind == TokenType::Identifier {
                let fn_name = t.text;
                //读取()
                let t1 = self.next();
                if t1.text == "(" {
                    let t2 = self.next();
                    if t2.text == ")" {
                        if let Some(body) = self.parse_fn_body() {
                            return Some(Statement::fn_decl(fn_name, body));
                        }
                    } else {
                        panic!("Expecting ')' in FunctionDecl, while we got a {}", t2.text);
                    }
                } else {
                    panic!("Expecting '(' in FunctionDecl, while we got a {}", t1.text);
                }
            }
        }
        //如果解析不成功，回溯
        self.trace_back(old_pos);
        None
    }

    pub fn parse_fn_call(&mut self) -> Option<Statement> {
        let old_pos = self.position();
        let t = self.next();
        let mut params: Vec<&'static str> = Vec::new();
        if t.kind == TokenType::Identifier {
            let fn_name = t.text;
            let t1 = self.next();
            if t1.text == "(" {
                let mut t2 = self.next();
                while t2.text != ")" {
                    if t2.kind == TokenType::StringLiteral {
                        params.push(t2.text);
                    } else {
                        panic!(
                            "Expecting parameter in FunctionCall, while we got a {}",
                            t2.text,
                        );
                    }
                    t2 = self.next();
                    if t2.text != ")" {
                        if t2.text == "," {
                            t2 = self.next();
                        } else {
                            panic!(
                                "Expecting a comma in FunctionCall, while we got a {}",
                                t2.text
                            );
                        }
                    }
                }
                //消化掉一个分号：;
                t2 = self.next();
                if t2.text == ";" {
                    return Some(Statement::fn_call(fn_name, params));
                } else {
                    panic!(
                        "Expecting a comma in FunctionCall, while we got a {}",
                        t2.text
                    );
                }
            }
        }
        //如果解析不成功，回溯，返回null。
        self.trace_back(old_pos);
        None
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

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let tokenizer = Tokenizer::new(tokens.to_vec());
    let mut parser = Parser::new(tokenizer);
    let prog = parser.parse_prog();
    prog.dump();
}
