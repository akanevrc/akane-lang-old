#[cfg(test)]
mod tests;

use std::iter::Peekable;
use anyhow::{
    Error,
    Result
};
use crate::data::*;
use crate::bail_info;

pub fn lex<'input>(input: &'input str) -> Result<Vec<TokenInfo<'input>>, Vec<Error>> {
    let mut tokens = Vec::new();
    let mut errs = Vec::new();
    let mut str_iter = StrInfoIter::new(input).peekable();
    loop {
        match assume(&mut str_iter) {
            Ok(Some(Some(token))) =>
                tokens.push(token),
            Ok(Some(None)) =>
                (),
            Ok(None) => {
                if let Some(TokenInfo(last, info)) = tokens.last() {
                    if *last != semicolon() {
                        tokens.push(TokenInfo::new(semicolon(), info.clone()));
                    }
                }
                break;
            },
            Err(e) => {
                errs.push(e);
                str_iter.next();
            },
        }
    }
    if errs.len() == 0 {
        Ok(tokens)
    }
    else {
        Err(errs)
    }
}

fn assume<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<Option<TokenInfo<'input>>>> {
    if let Some(_) = assume_eof(str_iter)? {
        Ok(None)
    }
    else if let Some(_) = assume_whitespace(str_iter)? {
        Ok(Some(None))
    }
    else if let Some(token) = assume_token(str_iter)? {
        Ok(Some(Some(token)))
    }
    else {
        let (info, c) = &str_iter.peek().unwrap();
        bail_info!(info, "Invalid token found: `{}`", c);
    }
}

fn assume_eof<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<()>> {
    if str_iter.peek().is_none() {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_whitespace<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<()>> {
    let mut consumed = false;
    while is_whitespace(str_iter.peek()) {
        str_iter.next();
        consumed = true;
    }
    if consumed {
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_token<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if let Some(token) = assume_semicolon(str_iter)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_keyword_or_ident(str_iter)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_int(str_iter)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_paren(str_iter)? {
        Ok(Some(token))
    }
    else if let Some(token) = assume_symbol_or_op_code(str_iter)? {
        Ok(Some(token))
    }
    else {
        Ok(None)
    }
}

fn assume_semicolon<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if is_semicolon(str_iter.peek()) {
        let (info, _) = str_iter.next().unwrap();
        Ok(Some(TokenInfo::new(semicolon(), info)))
    }
    else {
        Ok(None)
    }
}

fn assume_keyword_or_ident<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if is_ident_head(str_iter.peek()) {
        let (info_head, c) = str_iter.next().unwrap();
        let mut token = String::from(c);
        let mut info_tail = info_head.clone();
        while is_ident_tail(str_iter.peek()) {
            let (info, c) = str_iter.next().unwrap();
            token.push(c);
            info_tail = info;
        }
        let info = info_head.extend(&info_tail);
        if is_ty(&token) {
            Ok(Some(TokenInfo::new(ty_keyword(), info)))
        }
        else if is_fn(&token) {
            Ok(Some(TokenInfo::new(fn_keyword(), info)))
        }
        else {
            Ok(Some(TokenInfo::new(ident(token), info)))
        }
    }
    else {
        Ok(None)
    }
}

fn assume_int<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if is_num(str_iter.peek()) {
        let (info_head, c) = str_iter.next().unwrap();
        let mut token = String::from(c);
        let mut info_tail = info_head.clone();
        while is_num(str_iter.peek()) {
            let (info, c) = str_iter.next().unwrap();
            token.push(c);
            info_tail = info;
        }
        let info = info_head.extend(&info_tail);
        Ok(Some(TokenInfo::new(int(token), info)))
    }
    else {
        Ok(None)
    }
}

fn assume_paren<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<TokenInfo<'input>>> {
    let s = str_iter.peek();
    if is_l_paren(s) {
        let (info, _) = str_iter.next().unwrap();
        Ok(Some(TokenInfo::new(l_paren(), info)))
    }
    else if is_r_paren(s) {
        let (info, _) = str_iter.next().unwrap();
        Ok(Some(TokenInfo::new(r_paren(), info)))
    }
    else {
        Ok(None)
    }
}

fn assume_symbol_or_op_code<'input>(str_iter: &mut Peekable<StrInfoIter<'input>>) -> Result<Option<TokenInfo<'input>>> {
    if is_op_code(str_iter.peek()) {
        let (info_head, c) = str_iter.next().unwrap();
        let mut token = String::from(c);
        let mut info_tail = info_head.clone();
        while is_op_code(str_iter.peek()) {
            let (info, c) = str_iter.next().unwrap();
            token.push(c);
            info_tail = info;
        }
        let info = info_head.extend(&info_tail);
        if is_arrow(&token) {
            Ok(Some(TokenInfo::new(arrow(), info)))
        }
        else if is_equal(&token) {
            Ok(Some(TokenInfo::new(equal(), info)))
        }
        else {
            Ok(Some(TokenInfo::new(op_code(token), info)))
        }
    }
    else {
        Ok(None)
    }
}

fn is_whitespace<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| c.is_ascii_whitespace())
}

fn is_semicolon<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| *c == ';')
}

fn is_ident_head<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| *c == '_' || c.is_ascii_alphabetic())
}

fn is_ident_tail<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| *c == '_' || c.is_ascii_alphanumeric())
}

fn is_num<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| c.is_ascii_digit())
}

fn is_op_code<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| [
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

fn is_l_paren<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| *c == '(')
}

fn is_r_paren<'input>(c: Option<&(StrInfo<'input>, char)>) -> bool {
    c.map_or(false, |(_, c)| *c == ')')
}

fn is_ty(s: &str) -> bool {
    s == "ty"
}

fn is_fn(s: &str) -> bool {
    s == "fn"
}

fn is_arrow(s: &str) -> bool {
    s == "->"
}

fn is_equal(s: &str) -> bool {
    s == "="
}
