mod str_info;
mod str_info_iterator;
mod ctor;

pub use str_info::*;
pub use str_info_iterator::*;
pub use ctor::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo<'input>(
    pub Token,
    pub StrInfo<'input>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
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

impl<'input> TokenInfo<'input> {
    pub fn new(token: Token, info: StrInfo<'input>) -> Self {
        Self(token, info)
    }
}
