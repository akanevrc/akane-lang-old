use super::Token;

fn lex(input: &str) -> Vec<Token> {
    super::lex(input.to_owned()).unwrap()
}

fn eof() -> Token {
    Token::Eof
}

fn semicolon() -> Token {
    Token::Semicolon
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
fn lex_semicolon() {
    assert_eq!(lex(";"), &[semicolon(), eof()]);
}

#[test]
fn lex_keyword_or_ident() {
    assert_eq!(lex("_"), &[ident("_"), semicolon(), eof()]);
    assert_eq!(lex("a"), &[ident("a"), semicolon(), eof()]);
    assert_eq!(lex("A"), &[ident("A"), semicolon(), eof()]);
    assert_eq!(lex("あ"), &[ident("あ"), semicolon(), eof()]);
    assert_eq!(lex("AbcDef_123"), &[ident("AbcDef_123"), semicolon(), eof()]);
}

#[test]
fn lex_num() {
    assert_eq!(lex("0"), &[num("0"), semicolon(), eof()]);
    assert_eq!(lex("1234567890"), &[num("1234567890"), semicolon(), eof()]);
}

#[test]
fn lex_paren() {
    assert_eq!(lex("("), &[l_paren(), semicolon(), eof()]);
    assert_eq!(lex(")"), &[r_paren(), semicolon(), eof()]);
}

#[test]
fn lex_symbol_or_op_code() {
    assert_eq!(lex("="), &[equal(), semicolon(), eof()]);
    assert_eq!(lex("=="), &[op_code("=="), semicolon(), eof()]);
    assert_eq!(lex("+"), &[op_code("+"), semicolon(), eof()]);
    assert_eq!(lex(">>="), &[op_code(">>="), semicolon(), eof()]);
}

#[test]
fn lex_statement() {
    assert_eq!(
        lex("f a b c = a >>= (b >>= c)"),
        &[
            ident("f"),
            ident("a"),
            ident("b"),
            ident("c"),
            equal(),
            ident("a"),
            op_code(">>="),
            l_paren(),
            ident("b"),
            op_code(">>="),
            ident("c"),
            r_paren(),
            semicolon(),
            eof(),
        ]
    );
}
