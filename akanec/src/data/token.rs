
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Ident(String),
    Num(String),
    OpCode(String),
    Equal,
    LParen,
    RParen,
}
