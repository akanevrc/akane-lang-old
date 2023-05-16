use std::iter::Peekable;
use anyhow::{
    bail,
    Result,
};

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

pub fn lex(input: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    loop {
        if let Some(token) = assume_eof(&mut chars)? {
            tokens.push(token);
            return Ok(tokens);
        }
        assume_whitespace(&mut chars)?;
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
    use super::Token;

    fn lex(input: &str) -> Vec<Token> {
        super::lex(input.to_owned()).unwrap()
    }

    fn eof() -> Token {
        Token::Eof
    }

    fn ident(s: &str) -> Token {
        Token::Ident(s.to_owned())
    }

    fn num(s: &str) -> Token {
        Token::Num(s.to_owned())
    }

    fn op_code(s: &str) -> Token {
        Token::OpCode(s.to_owned())
    }

    fn equal() -> Token {
        Token::Equal
    }

    fn l_paren() -> Token {
        Token::LParen
    }

    fn r_paren() -> Token {
        Token::RParen
    }

    #[test]
    fn lex_eof() {
        assert_eq!(lex(""), &[eof()]);
    }

    #[test]
    fn lex_keyword_or_ident() {
        assert_eq!(lex("_"), &[ident("_"), eof()]);
        assert_eq!(lex("a"), &[ident("a"), eof()]);
        assert_eq!(lex("A"), &[ident("A"), eof()]);
        assert_eq!(lex("あ"), &[ident("あ"), eof()]);
        assert_eq!(lex("AbcDef_123"), &[ident("AbcDef_123"), eof()]);
    }

    #[test]
    fn lex_num() {
        assert_eq!(lex("0"), &[num("0"), eof()]);
        assert_eq!(lex("1234567890"), &[num("1234567890"), eof()]);
    }

    #[test]
    fn lex_paren() {
        assert_eq!(lex("("), &[l_paren(), eof()]);
        assert_eq!(lex(")"), &[r_paren(), eof()]);
    }

    #[test]
    fn symbol_or_op_code() {
        assert_eq!(lex("="), &[equal(), eof()]);
        assert_eq!(lex("=="), &[op_code("=="), eof()]);
        assert_eq!(lex("+"), &[op_code("+"), eof()]);
        assert_eq!(lex(">>="), &[op_code(">>="), eof()]);
    }
}
