
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Semicolon,
    Ident(String),
    Num(String),
    OpCode(String),
    Equal,
    LParen,
    RParen,
}
