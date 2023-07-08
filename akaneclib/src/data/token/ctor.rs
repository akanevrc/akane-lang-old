use crate::data::*;

pub fn semicolon() -> Token {
    Token::Semicolon
}

pub fn ident(s: String) -> Token {
    Token::Ident(s)
}

pub fn int(s: String) -> Token {
    Token::Int(s)
}

pub fn float(s: String) -> Token {
    Token::Float(s)
}

pub fn op_code(s: String) -> Token {
    Token::OpCode(s)
}

pub fn ty_keyword() -> Token {
    Token::Ty
}

pub fn fn_keyword() -> Token {
    Token::Fn
}

pub fn arrow() -> Token {
    Token::Arrow
}

pub fn equal() -> Token {
    Token::Equal
}

pub fn l_paren() -> Token {
    Token::LParen
}

pub fn r_paren() -> Token {
    Token::RParen
}
