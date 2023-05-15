use std::iter::Peekable;
use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst {
    left_def: LeftDefAst,
    expr: ExprAst,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftDefAst {
    ident: IdentAst,
    args: Vec<IdentAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprAst {
    Fn(FnAst),
    PrefixOp(PrefixOpAst),
    InfixOp(InfixOpAst),
    Ident(IdentAst),
    Num(NumAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnAst {
    fn_expr: Rc<ExprAst>,
    arg_expr: Rc<ExprAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixOpAst {
    op_code: String,
    rhs: Rc<ExprAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfixOpAst {
    op_code: String,
    lhs: Rc<ExprAst>,
    rhs: Rc<ExprAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentAst {
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumAst {
    value: String,
}

pub fn parse(input: Vec<Token>) -> Result<Vec<FnDefAst>> {
    let mut asts = Vec::new();
    let mut tokens = input.into_iter().peekable();
    loop {
        if let Some(_) = assume_eof(&mut tokens)? {
            return Ok(asts);
        }
        if let Some(ast) = assume_fn_def(&mut tokens)? {
            asts.push(ast);
            continue;
        }
        bail!("Invalid function definition.");
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

fn assume_fn_def(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<FnDefAst>> {
    if let Some(left_def) = assume_left_def(tokens)? {
        if let Some(_) = assume_equal(tokens)? {
            if let Some(expr) = assume_expr(tokens)? {
                return Ok(Some(FnDefAst { left_def, expr }));
            }
            bail!("Expression required.");
        }
        bail!("Equal required.");
    }
    else {
        Ok(None)
    }
}

fn assume_left_def(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<LeftDefAst>> {
    if let Some(ident) = assume_ident(tokens)? {
        let mut args = Vec::new();
        loop {
            if let Some(arg) = assume_ident(tokens)? {
                args.push(arg);
                continue;
            }
            return Ok(Some(LeftDefAst { ident, args }));
        }
    }
    else {
        Ok(None)
    }
}

fn assume_equal(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<()>> {
    if let Some(Token::Equal) = tokens.peek() {
        tokens.next();
        Ok(Some(()))
    }
    else {
        Ok(None)
    }
}

fn assume_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<ExprAst>> {
    if let Some(lhs) = assume_term(tokens)? {
        let mut lhs = lhs;
        while let Some((op_code, rhs)) = assume_infix_op_rhs(tokens)? {
            lhs = ExprAst::InfixOp(InfixOpAst { op_code, lhs: Rc::new(lhs), rhs: Rc::new(rhs) });
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
            term = ExprAst::Fn(FnAst { fn_expr: Rc::new(term), arg_expr: Rc::new(f) })
        }
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
    Ok(
        assume_paren(tokens)?
        .or(assume_ident(tokens)?.map(ExprAst::Ident))
        .or(assume_num(tokens)?.map(ExprAst::Num))
    )
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
        Ok(Some(IdentAst { name }))
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

fn assume_prefix_op(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Option<PrefixOpAst>> {
    panic!("Not implemented.");
}
