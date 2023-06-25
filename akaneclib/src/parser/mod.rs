#[cfg(test)]
mod tests;

use std::{
    iter::Peekable,
    rc::Rc,
};
use anyhow::{
    Error,
    Result,
};
use crate::data::*;
use crate::bail_info;

macro_rules! bail_tokens_with_line {
    ($tokens:expr, $msg:literal) => {
        {
            let info = &$tokens.peek().unwrap().1;
            let target_part_of_line = format!("\n{}", info.target_part_of_line());
            bail_info!(info, $msg, target_part_of_line)
        }
    };
}

pub fn parse<'input>(input: Vec<TokenInfo<'input>>) -> Result<Vec<TopDefEnum<'input>>, Vec<Error>> {
    let mut asts = Vec::new();
    let mut errs = Vec::new();
    let mut tokens = input.into_iter().peekable();
    loop {
        match assume(&mut tokens) {
            Ok(Some(ast)) =>
                asts.push(ast.clone()),
            Ok(None) =>
                break,
            Err(e) => {
                errs.push(e);
                tokens.next();
            },
        }
    }
    if errs.len() == 0 {
        Ok(asts)
    }
    else {
        Err(errs)
    }
}

fn assume<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TopDefEnum<'input>>> {
    if let Some(_) = assume_eof(tokens)? {
        Ok(None)
    }
    else if let Some(ast) = assume_top_def(tokens)? {
        Ok(Some(ast))
    }
    else {
        bail_tokens_with_line!(tokens, "Invalid top-level definition:{}");
    }
}

fn assume_eof<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<()>> {
    if let Some(_) = tokens.peek() {
        Ok(None)
    }
    else {
        Ok(Some(()))
    }
}

fn assume_top_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TopDefEnum<'input>>> {
    if let Some(ast) = assume_fn_def(tokens)? {
        if let Some(_) = assume_simple_token(tokens, semicolon())? {
            return Ok(Some(top_fn_def_ast(ast)));
        }
        bail_tokens_with_line!(tokens, "`;` required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<FnDefAst<'input>>> {
    let mut ty_annot = None;
    let mut ty_info = None;
    if let Some(info) = assume_simple_token(tokens, ty_keyword())? {
        ty_info = Some(info);
        if let Some(ty_expr) = assume_ty_expr(tokens)? {
            ty_annot = Some(ty_expr);
        }
    }
    if let Some(fn_info) = assume_simple_token(tokens, fn_keyword())? {
        if let Some(left_fn_def) = assume_left_fn_def(tokens)? {
            if let Some(_) = assume_simple_token(tokens, equal())? {
                if let Some(expr) = assume_expr(tokens)? {
                    let extended =
                        if let Some(ty_info) = ty_info {
                            ty_info.extend(&expr.str_info)
                        }
                        else {
                            fn_info.extend(&expr.str_info)
                        };
                    return Ok(Some(fn_def_ast(ty_annot, left_fn_def, expr, extended)));
                }
                bail_tokens_with_line!(tokens, "Expression required:{}");
            }
            bail_tokens_with_line!(tokens, "`=` required:{}");
        }
        bail_tokens_with_line!(tokens, "Left function definition required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_ty_expr<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    let mut exprs = Vec::new();
    if let Some(lhs) = assume_ty_lhs(tokens)? {
        exprs.push(lhs);
        while let Some(rhs) = assume_ty_rhs(tokens)? {
            exprs.push(rhs);
        }
        let mut expr_iter = exprs.into_iter().rev();
        let mut rhs = expr_iter.next().unwrap();
        for lhs in expr_iter {
            let extended = lhs.str_info.extend(&rhs.str_info);
            rhs = ty_arrow_expr_ast(ty_arrow_ast(lhs, rhs, extended.clone()), extended);
        }
        Ok(Some(rhs))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(term) = assume_ty_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(TokenInfo(Token::Arrow, _)) = tokens.peek() {
        tokens.next();
        if let Some(term) = assume_ty_term(tokens)? {
            return Ok(Some(term));
        }
        bail_tokens_with_line!(tokens, "Type term required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_ty_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(factor) = assume_ty_factor(tokens)? {
        Ok(Some(factor))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>> {
    if let Some(expr) = assume_ty_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ty_ident(tokens)? {
        Ok(Some(ty_ident_expr_ast(ident.clone(), ident.str_info.clone())))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst<'input>>>>  {
    if let Some(TokenInfo(Token::LParen, _)) = tokens.peek() {
        tokens.next();
        if let Some(expr) = assume_ty_expr(tokens)? {
            if let Some(TokenInfo(Token::RParen, _)) = tokens.peek() {
                tokens.next();
                return Ok(Some(expr))
            }
            bail_tokens_with_line!(tokens, "`)` required:{}")
        }
        bail_tokens_with_line!(tokens, "Expression required:{}")
    }
    else {
        Ok(None)
    }
}

fn assume_ty_ident<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TyIdentAst<'input>>> {
    if let Some(TokenInfo(Token::Ident(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(ty_ident_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_left_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<LeftFnDefAst<'input>>> {
    if let Some(ident) = assume_ident(tokens)? {
        let mut args = Vec::new();
        loop {
            if let Some(arg) = assume_ident(tokens)? {
                args.push(arg);
                continue;
            }
            let extended =
                if let Some(last) = args.last() {
                    ident.str_info.extend(&last.str_info)
                }
                else {
                    ident.str_info
                };
            let names = args.into_iter().map(|arg| arg.name).collect();
            return Ok(Some(left_fn_def_ast(ident.name, names, extended)));
        }
    }
    else {
        Ok(None)
    }
}

fn assume_expr<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(lhs) = assume_prefix_op_lhs(tokens)? {
        let mut lhs = lhs;
        while let Some((op_code, info, rhs)) = assume_infix_op_rhs(tokens)? {
            let name = infix_op_name(&op_code, tokens)?;
            let extended = lhs.str_info.extend(&rhs.str_info);
            lhs = fn_expr_ast(infix_op_ast(name, lhs.clone(), rhs, extended.clone(), info, lhs.str_info.clone()), extended);
        }
        Ok(Some(lhs))
    }
    else {
        Ok(None)
    }
}

fn assume_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(factor) = assume_factor(tokens)? {
        let mut term = factor.clone();
        while let Some(f) = assume_factor(tokens)? {
            let extended = factor.str_info.extend(&f.str_info);
            term = fn_expr_ast(fn_ast(term, f, extended.clone()), extended);
        }
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_prefix_op_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(TokenInfo(Token::OpCode(op_code), info)) = tokens.peek() {
        let op_code = op_code.clone();
        let info = info.clone();
        tokens.next();
        if let Some(term) = assume_term(tokens)? {
            let name = prefix_op_name(&op_code, tokens)?;
            let extended = info.extend(&term.str_info);
            return Ok(Some(fn_expr_ast(prefix_op_ast(name, term, extended.clone(), info), extended)));
        }
        bail_tokens_with_line!(tokens, "Term required:{}");
    }
    else if let Some(term) = assume_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_infix_op_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<(String, StrInfo<'input>, Rc<ExprAst<'input>>)>> {
    if let Some(TokenInfo(Token::OpCode(op_code), info)) = tokens.peek() {
        let op_code = op_code.clone();
        let info = info.clone();
        tokens.next();
        if let Some(term) = assume_term(tokens)? {
            return Ok(Some((op_code, info, term)));
        }
        bail_tokens_with_line!(tokens, "Term required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>> {
    if let Some(expr) = assume_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ident(tokens)? {
        Ok(Some(ident_expr_ast(ident.clone(), ident.str_info)))
    }
    else if let Some(num) = assume_num(tokens)? {
        Ok(Some(ident_expr_ast(num.clone(), num.str_info)))
    }
    else {
        Ok(None)
    }
}

fn assume_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst<'input>>>>  {
    if let Some(TokenInfo(Token::LParen, _)) = tokens.peek() {
        tokens.next();
        if let Some(expr) = assume_expr(tokens)? {
            if let Some(TokenInfo(Token::RParen, _)) = tokens.peek() {
                tokens.next();
                return Ok(Some(expr))
            }
            bail_tokens_with_line!(tokens, "`)` required:{}")
        }
        bail_tokens_with_line!(tokens, "Expression required:{}")
    }
    else {
        Ok(None)
    }
}

fn assume_ident<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IdentAst<'input>>> {
    if let Some(TokenInfo(Token::Ident(name), info)) = tokens.peek() {
        let name = name.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(ident_ast(name, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_num<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IdentAst<'input>>> {
    if let Some(TokenInfo(Token::Num(value), info)) = tokens.peek() {
        let value = value.clone();
        let info = info.clone();
        tokens.next();
        Ok(Some(ident_ast(value, info)))
    }
    else {
        Ok(None)
    }
}

fn assume_simple_token<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>, assumed: Token) -> Result<Option<StrInfo<'input>>> {
    if let Some(TokenInfo(token, info)) = tokens.peek() {
        if *token == assumed {
            let info = info.clone();
            tokens.next();
            Ok(Some(info))
        }
        else {
            Ok(None)
        }
    }
    else {
        Ok(None)
    }
}

fn prefix_op_name<'input>(op_code: &str, tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<String> {
    if op_code == "-" {
        Ok("negate".to_owned())
    }
    else {
        bail_tokens_with_line!(tokens, "Invalid prefix operator:{}");
    }
}

fn infix_op_name<'input>(op_code: &str, tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<String> {
    if op_code == "+" {
        Ok("add".to_owned())
    }
    else {
        bail_tokens_with_line!(tokens, "Invalid infix operator:{}");
    }
}
