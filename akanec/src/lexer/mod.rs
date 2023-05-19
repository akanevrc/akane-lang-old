#[cfg(test)]
mod tests;

use std::iter::Peekable;
use anyhow::{
    bail,
    Result,
};
use crate::data::token::Token;

pub fn lex(input: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    loop {
        if let Some(token) = assume_eof(&mut chars)? {
            tokens.push(token);
            return Ok(tokens);
        }
        if let Some(_) = assume_whitespace(&mut chars)? {
            continue;
        }
        if let Some(token) = assume_token(&mut chars)? {
            tokens.push(token);
            continue;
        }
        bail!("No valid token found.")
    }
}

fn assume_eof(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    if chars.peek().is_none() {
        Ok(Some(Token::Eof))
    }
    else {
        Ok(None)
    }
}

fn assume_whitespace(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<()>> {
    let mut consumed = false;
    while is_whitespace(chars.peek()) {
        chars.next();
        consumed = true;
    }
    if consumed {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_token(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    if let Some(token) = assume_keyword_or_ident(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_num(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_paren(chars)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_symbol_or_op_code(chars)? {
        Ok(Some(token))
    }
    else {
        Ok(None)
    }
}

fn assume_keyword_or_ident(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    if is_ident_head(chars.peek()) {
        let mut token = String::from(chars.next().unwrap());
        while is_ident_tail(chars.peek()) {
            token.push(chars.next().unwrap());
        }
        Ok(Some(Token::Ident(token)))
    }
    else {
        Ok(None)
    }
}

fn assume_num(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    if is_num(chars.peek()) {
        let mut token = String::from(chars.next().unwrap());
        while is_num(chars.peek()) {
            token.push(chars.next().unwrap());
        }
        Ok(Some(Token::Num(token)))
    }
    else {
        Ok(None)
    }
}

fn assume_paren(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    let c = chars.peek();
    if is_l_paren(c) {
        chars.next();
        Ok(Some(Token::LParen))
    }
    else if is_r_paren(c) {
        chars.next();
        Ok(Some(Token::RParen))
    }
    else {
        Ok(None)
    }
}

fn assume_symbol_or_op_code(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    if is_op_code(chars.peek()) {
        let mut token = String::from(chars.next().unwrap());
        while is_op_code(chars.peek()) {
            token.push(chars.next().unwrap());
        }
        if is_equal(&mut token) {
            Ok(Some(Token::Equal))
        }
        else {
            Ok(Some(Token::OpCode(token)))
        }
    }
    else {
        Ok(None)
    }
}

fn is_whitespace(c: Option<&char>) -> bool {
    c.map_or(false, |c| c.is_whitespace())
}

fn is_ident_head(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == '_' || c.is_alphabetic())
}

fn is_ident_tail(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == '_' || c.is_alphanumeric())
}

fn is_num(c: Option<&char>) -> bool {
    c.map_or(false, |c| c.is_numeric())
}

fn is_op_code(c: Option<&char>) -> bool {
    c.map_or(false, |c| [
        '!',
        '#',
        '$',
        '%',
        '&',
        '*',
        '+',
        '.',
        '/',
        '<',
        '=',
        '>',
        '?',
        '@',
        '\\',
        '^',
        '|',
        '-',
        '~',
    ].contains(c))
}

fn is_l_paren(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == '(')
}

fn is_r_paren(c: Option<&char>) -> bool {
    c.map_or(false, |c| *c == ')')
}

fn is_equal(s: &str) -> bool {
    s == "="
}
