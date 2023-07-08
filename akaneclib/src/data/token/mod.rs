mod ctor;

pub use ctor::*;

use crate::data::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo<'input>(
    pub Token,
    pub StrInfo<'input>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Semicolon,
    Ident(String),
    Int(String),
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
