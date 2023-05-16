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
