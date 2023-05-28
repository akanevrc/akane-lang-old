#[cfg(test)]
mod tests;

use std::{
    cell::RefCell,
    iter::Peekable,
    rc::Rc,
};
use anyhow::{
    bail,
    Result,
};
use crate::data::{
    token::Token,
    ast::{
        TopDefEnum,
        FnDefAst,
        LeftFnDefAst,
        ExprAst,
        ExprEnum,
        FnAst,
        PrefixOpAst,
        InfixOpAst,
        IdentAst,
        NumAst,
    },
};

pub fn parse(input: Vec<Token>) -> Result<Vec<TopDefEnum>> {
    let mut asts = Vec::new();
    let mut tokens = input.into_iter().peekable();
    loop {
        if let Some(_) = assume_eof(&mut tokens)? {
            return Ok(asts);
        }
        if let Some(ast) = assume_top_def(&mut tokens)? {
            asts.push(ast);
            continue;
        }
        bail!("Invalid top-level definition.");
    }
}

fn assume_eof(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<()>> {
    if let Some(Token::Eof) = tokens.peek() {
        tokens.next();
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_top_def(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<TopDefEnum>> {
    if let Some(ast) = assume_fn_def(tokens)? {
        Ok(Some(TopDefEnum::Fn(ast)))
    }
    else {
        Ok(None)
    }
}

fn assume_fn_def(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<FnDefAst>> {
    if let Some(_) = assume_simple_token(tokens, Token::Fn)? {
        if let Some(left_fn_def) = assume_left_fn_def(tokens)? {
            if let Some(_) = assume_simple_token(tokens, Token::Equal)? {
                if let Some(expr) = assume_expr(tokens)? {
                    if let Some(_) = assume_simple_token(tokens, Token::Semicolon)? {
                        return Ok(Some(FnDefAst { left_fn_def, expr, fn_sem: RefCell::new(None), arg_sems: RefCell::new(None) }));
                    }
                }
                bail!("Expression required.");
            }
            bail!("Equal required.");
        }
        bail!("Left function definition required.");
    }
    else {
        Ok(None)
    }
}

fn assume_left_fn_def(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<LeftFnDefAst>> {
    if let Some(ident) = assume_ident(tokens)? {
        let mut args = Vec::new();
        loop {
            if let Some(arg) = assume_ident(tokens)? {
                args.push(arg.name);
                continue;
            }
            return Ok(Some(LeftFnDefAst { name: ident.name, args }));
        }
    }
    else {
        Ok(None)
    }
}

fn assume_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<ExprAst>> {
    if let Some(lhs) = assume_prefix_op_lhs(tokens)? {
        let mut lhs = lhs;
        while let Some((op_code, rhs)) = assume_infix_op_rhs(tokens)? {
            lhs = ExprAst {
                expr_enum: ExprEnum::InfixOp(
                    InfixOpAst { op_code, lhs: Rc::new(lhs), rhs: Rc::new(rhs) }
                ),
                ty_sem: RefCell::new(None)
            };
        }
        Ok(Some(lhs))
    }
    else {
        Ok(None)
    }
}

fn assume_term(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<ExprAst>> {
    if let Some(factor) = assume_factor(tokens)? {
        let mut term = factor;
        while let Some(f) = assume_factor(tokens)? {
            term = ExprAst {
                expr_enum: ExprEnum::Fn(
                    FnAst { fn_expr: Rc::new(term), arg_expr: Rc::new(f), thunk: RefCell::new(None) }
                ),
                ty_sem: RefCell::new(None)
            };
        }
        Ok(Some(term))
    }
    else {
        Ok(None)
    }
}

fn assume_prefix_op_lhs(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<ExprAst>> {
    if let Some(Token::OpCode(op_code)) = tokens.peek() {
        let op_code = op_code.to_owned();
        if op_code == "-" {
            tokens.next();
            if let Some(term) = assume_term(tokens)? {
                return Ok(Some(ExprAst {
                    expr_enum: ExprEnum::PrefixOp(
                        PrefixOpAst { op_code, rhs: Rc::new(term) }
                    ),
                    ty_sem: RefCell::new(None)
                }));
            }
            bail!("Term required.");
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

fn assume_infix_op_rhs(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<(String, ExprAst)>> {
    if let Some(Token::OpCode(op_code)) = tokens.peek() {
        let op_code = op_code.to_owned();
        tokens.next();
        if let Some(term) = assume_term(tokens)? {
            return Ok(Some((op_code, term)));
        }
        bail!("Term required.");
    }
    else {
        Ok(None)
    }
}

fn assume_factor(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<ExprAst>> {
    if let Some(expr) = assume_paren(tokens)? {
        Ok(Some(expr))
    }
    else if let Some(ident) = assume_ident(tokens)? {
        Ok(Some(ExprAst { expr_enum: ExprEnum::Ident(ident), ty_sem: RefCell::new(None) }))
    }
    else if let Some(num) = assume_num(tokens)? {
        Ok(Some(ExprAst { expr_enum: ExprEnum::Num(num), ty_sem: RefCell::new(None) }))
    }
    else {
        Ok(None)
    }
}

fn assume_paren(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<ExprAst>>  {
    if let Some(Token::LParen) = tokens.peek() {
        tokens.next();
        if let Some(expr) = assume_expr(tokens)? {
            if let Some(Token::RParen) = tokens.peek() {
                tokens.next();
                return Ok(Some(expr))
            }
            bail!("Right paren required.")
        }
        bail!("Expression required.")
    }
    else {
        Ok(None)
    }
}

fn assume_ident(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<IdentAst>> {
    if let Some(Token::Ident(name)) = tokens.peek() {
        let name = name.to_owned();
        tokens.next();
        Ok(Some(IdentAst { name, thunk: RefCell::new(None) }))
    }
    else {
        Ok(None)
    }
}

fn assume_num(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<NumAst>> {
    if let Some(Token::Num(value)) = tokens.peek() {
        let value = value.to_owned();
        tokens.next();
        Ok(Some(NumAst { value }))
    }
    else {
        Ok(None)
    }
}

fn assume_simple_token(tokens: &mut Peekable<impl Iterator<Item = Token>>, assumed: Token) -> Result<Option<()>> {
    if let Some(token) = tokens.peek() {
        if *token == assumed {
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
