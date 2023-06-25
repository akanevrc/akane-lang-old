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

pub fn parse<'input>(input: Vec<TokenInfo<'input>>) -> Result<Vec<TopDefEnum>, Vec<Error>> {
    let mut asts = Vec::new();
    let mut errs = Vec::new();
    let mut tokens = input.into_iter().peekable();
    loop {
        match assume(&mut tokens) {
            Ok(Some(ast)) =>
                asts.push(ast),
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

fn assume<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TopDefEnum>> {
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

fn assume_top_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TopDefEnum>> {
    let ret =
        if let Some(ast) = assume_fn_def(tokens)? {
            Ok(Some(top_fn_def_ast(ast)))
        }
        else {
            return Ok(None)
        };
    if let Some(_) = assume_simple_token(tokens, semicolon())? {
        ret
    }
    else {
        bail_tokens_with_line!(tokens, "`;` required:{}");
    }
}

fn assume_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<FnDefAst>> {
    let mut ty_annot = None;
    if let Some(_) = assume_simple_token(tokens, ty_keyword())? {
        if let Some(ty_expr) = assume_ty_expr(tokens)? {
            ty_annot = Some(ty_expr);
        }
    }
    if let Some(_) = assume_simple_token(tokens, fn_keyword())? {
        if let Some(left_fn_def) = assume_left_fn_def(tokens)? {
            if let Some(_) = assume_simple_token(tokens, equal())? {
                if let Some(expr) = assume_expr(tokens)? {
                    return Ok(Some(fn_def_ast(ty_annot, left_fn_def, expr)));
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

fn assume_ty_expr<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst>>> {
    let mut exprs = Vec::new();
    if let Some(lhs) = assume_ty_lhs(tokens)? {
        exprs.push(lhs);
        while let Some(rhs) = assume_ty_rhs(tokens)? {
            exprs.push(rhs);
        }
        let mut expr_iter = exprs.into_iter().rev();
        let mut rhs = expr_iter.next().unwrap();
        for lhs in expr_iter {
            rhs = ty_arrow_expr_ast(ty_arrow_ast(lhs, rhs));
        }
        Ok(Some(rhs))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst>>> {
    if let Some(term) = assume_ty_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst>>> {
    if let Some(Token::Arrow) = tokens.peek().map(|token| token.0.clone()) {
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

fn assume_ty_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst>>> {
    if let Some(factor) = assume_ty_factor(tokens)? {
        Ok(Some(factor))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst>>> {
    if let Some(expr) = assume_ty_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ty_ident(tokens)? {
        Ok(Some(ty_ident_expr_ast(ident)))
    }
    else {
        Ok(None)
    }
}

fn assume_ty_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<TyExprAst>>>  {
    if let Some(Token::LParen) = tokens.peek().map(|token| token.0.clone()) {
        tokens.next();
        if let Some(expr) = assume_ty_expr(tokens)? {
            if let Some(Token::RParen) = tokens.peek().map(|token| token.0.clone()) {
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

fn assume_ty_ident<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<TyIdentAst>> {
    if let Some(Token::Ident(name)) = tokens.peek().map(|token| token.0.clone()) {
        let name = name.to_owned();
        tokens.next();
        Ok(Some(ty_ident_ast(name)))
    }
    else {
        Ok(None)
    }
}

fn assume_left_fn_def<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<LeftFnDefAst>> {
    if let Some(ident) = assume_ident(tokens)? {
        let mut args = Vec::new();
        loop {
            if let Some(arg) = assume_ident(tokens)? {
                args.push(arg.name);
                continue;
            }
            return Ok(Some(left_fn_def_ast(ident.name, args)));
        }
    }
    else {
        Ok(None)
    }
}

fn assume_expr<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst>>> {
    if let Some(lhs) = assume_prefix_op_lhs(tokens)? {
        let mut lhs = lhs;
        while let Some((op_code, rhs)) = assume_infix_op_rhs(tokens)? {
            let name = infix_op_name(&op_code, tokens)?;
            lhs = fn_expr_ast(infix_op_ast(name, lhs, rhs));
        }
        Ok(Some(lhs))
    }
    else {
        Ok(None)
    }
}

fn assume_term<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst>>> {
    if let Some(factor) = assume_factor(tokens)? {
        let mut term = factor;
        while let Some(f) = assume_factor(tokens)? {
            term = fn_expr_ast(fn_ast(term, f));
        }
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_prefix_op_lhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst>>> {
    if let Some(Token::OpCode(op_code)) = tokens.peek().map(|token| token.0.clone()) {
        let op_code = op_code.to_owned();
        if op_code == "-" {
            tokens.next();
            if let Some(term) = assume_term(tokens)? {
                let name = prefix_op_name(&op_code, tokens)?;
                return Ok(Some(fn_expr_ast(prefix_op_ast(name, term))));
            }
            bail_tokens_with_line!(tokens, "Term required:{}");
        }
        Ok(None)
    }
    else if let Some(term) = assume_term(tokens)? {
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_infix_op_rhs<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<(String, Rc<ExprAst>)>> {
    if let Some(Token::OpCode(op_code)) = tokens.peek().map(|token| token.0.clone()) {
        let op_code = op_code.to_owned();
        tokens.next();
        if let Some(term) = assume_term(tokens)? {
            return Ok(Some((op_code, term)));
        }
        bail_tokens_with_line!(tokens, "Term required:{}");
    }
    else {
        Ok(None)
    }
}

fn assume_factor<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst>>> {
    if let Some(expr) = assume_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ident(tokens)? {
        Ok(Some(ident_expr_ast(ident)))
    }
    else if let Some(num) = assume_num(tokens)? {
        Ok(Some(ident_expr_ast(num)))
    }
    else {
        Ok(None)
    }
}

fn assume_paren<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<Rc<ExprAst>>>  {
    if let Some(Token::LParen) = tokens.peek().map(|token| token.0.clone()) {
        tokens.next();
        if let Some(expr) = assume_expr(tokens)? {
            if let Some(Token::RParen) = tokens.peek().map(|token| token.0.clone()) {
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

fn assume_ident<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IdentAst>> {
    if let Some(Token::Ident(name)) = tokens.peek().map(|token| token.0.clone()) {
        let name = name.to_owned();
        tokens.next();
        Ok(Some(ident_ast(name)))
    }
    else {
        Ok(None)
    }
}

fn assume_num<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>) -> Result<Option<IdentAst>> {
    if let Some(Token::Num(value)) = tokens.peek().map(|token| token.0.clone()) {
        let value = value.to_owned();
        tokens.next();
        Ok(Some(ident_ast(value)))
    }
    else {
        Ok(None)
    }
}

fn assume_simple_token<'input>(tokens: &mut Peekable<impl Iterator<Item = TokenInfo<'input>>>, assumed: Token) -> Result<Option<()>> {
    if let Some(token) = tokens.peek().map(|token| token.0.clone()) {
        if token == assumed {
            tokens.next();
            Ok(Some(()))
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
