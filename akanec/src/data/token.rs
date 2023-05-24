
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Semicolon,
    Ident(String),
    Num(String),
    OpCode(String),
    Fn,
    Equal,
    LParen,
    RParen,
}
