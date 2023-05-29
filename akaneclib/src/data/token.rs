
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Semicolon,
    Ident(String),
    Num(String),
    OpCode(String),
    Ty,
    Fn,
    Arrow,
    Equal,
    LParen,
    RParen,
}
