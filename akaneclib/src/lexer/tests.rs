use crate::data::*;

fn lex(input: &str) -> Vec<Token> {
    super::lex(input).unwrap().into_iter().map(|token| token.0).collect()
}

fn ident(s: &str) -> Token {
    crate::data::ident(s.to_owned())
}

fn num(s: &str) -> Token {
    crate::data::num(s.to_owned())
}

fn op_code(s: &str) -> Token {
    crate::data::op_code(s.to_owned())
}

#[test]
fn lex_eof() {
    assert_eq!(lex(""), &[]);
}

#[test]
fn lex_semicolon() {
    assert_eq!(lex(";"), &[semicolon()]);
}

#[test]
fn lex_keyword_or_ident() {
    assert_eq!(lex("ty"), &[ty_keyword(), semicolon()]);
    assert_eq!(lex("fn"), &[fn_keyword(), semicolon()]);
    assert_eq!(lex("_"), &[ident("_"), semicolon()]);
    assert_eq!(lex("a"), &[ident("a"), semicolon()]);
    assert_eq!(lex("A"), &[ident("A"), semicolon()]);
    assert_eq!(lex("AbcDef_123"), &[ident("AbcDef_123"), semicolon()]);
}

#[test]
fn lex_num() {
    assert_eq!(lex("0"), &[num("0"), semicolon()]);
    assert_eq!(lex("1234567890"), &[num("1234567890"), semicolon()]);
}

#[test]
fn lex_paren() {
    assert_eq!(lex("("), &[l_paren(), semicolon()]);
    assert_eq!(lex(")"), &[r_paren(), semicolon()]);
}

#[test]
fn lex_symbol_or_op_code() {
    assert_eq!(lex("->"), &[arrow(), semicolon()]);
    assert_eq!(lex("="), &[equal(), semicolon()]);
    assert_eq!(lex("=="), &[op_code("=="), semicolon()]);
    assert_eq!(lex("+"), &[op_code("+"), semicolon()]);
    assert_eq!(lex(">>="), &[op_code(">>="), semicolon()]);
}

#[test]
fn lex_statement() {
    assert_eq!(
        lex("fn f a b c = a >>= (b >>= c)"),
        &[
            fn_keyword(),
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
        ]
    );
}
