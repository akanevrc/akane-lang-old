use std::iter::Peekable;
use anyhow::{
    bail,
    Result,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Eof,
    Ident(String),
    Num(String),
    OpCode(String),
    Equal,
    LParen,
    RParen,
}

pub fn lex(input: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    loop {
        if let Some(token) = assume_eof(&mut chars)? {
            tokens.push(token);
            return Ok(tokens);
        }
        assume_whitespace(&mut chars);
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

fn assume_whitespace(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<()> {
    while is_whitespace(chars.peek()) {
        chars.next();
    }
    Ok(())
}

fn assume_token(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Option<Token>> {
    Ok(
        assume_keyword_or_ident(chars)?
        .or(assume_num(chars)?)
        .or(assume_paren(chars)?)
        .or(assume_symbol_or_op_code(chars)?)
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_eof() {
        assert_eq!(lex("".to_owned()).unwrap(), &[Token::Eof]);
    }

    #[test]
    fn lex_keyword_or_ident() {
        assert_eq!(lex("_".to_owned()).unwrap(), &[Token::Ident("_".to_owned()), Token::Eof]);
        assert_eq!(lex("a".to_owned()).unwrap(), &[Token::Ident("a".to_owned()), Token::Eof]);
        assert_eq!(lex("A".to_owned()).unwrap(), &[Token::Ident("A".to_owned()), Token::Eof]);
        assert_eq!(lex("あ".to_owned()).unwrap(), &[Token::Ident("あ".to_owned()), Token::Eof]);
        assert_eq!(lex("AbcDef_123".to_owned()).unwrap(), &[Token::Ident("AbcDef_123".to_owned()), Token::Eof]);
    }

    #[test]
    fn lex_num() {
        assert_eq!(lex("0".to_owned()).unwrap(), &[Token::Num("0".to_owned()), Token::Eof]);
        assert_eq!(lex("1234567890".to_owned()).unwrap(), &[Token::Num("1234567890".to_owned()), Token::Eof]);
    }

    #[test]
    fn lex_paren() {
        assert_eq!(lex("(".to_owned()).unwrap(), &[Token::LParen, Token::Eof]);
        assert_eq!(lex(")".to_owned()).unwrap(), &[Token::RParen, Token::Eof]);
    }

    #[test]
    fn symbol_or_op_code() {
        assert_eq!(lex("=".to_owned()).unwrap(), &[Token::Equal, Token::Eof]);
        assert_eq!(lex("==".to_owned()).unwrap(), &[Token::OpCode("==".to_owned()), Token::Eof]);
        assert_eq!(lex("+".to_owned()).unwrap(), &[Token::OpCode("+".to_owned()), Token::Eof]);
        assert_eq!(lex(">>=".to_owned()).unwrap(), &[Token::OpCode(">>=".to_owned()), Token::Eof]);
    }
}
