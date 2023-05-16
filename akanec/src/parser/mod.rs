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

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use super::{
        FnDefAst,
        LeftDefAst,
        ExprAst,
        FnAst,
        PrefixOpAst,
        InfixOpAst,
        IdentAst,
        NumAst,
    };

    fn parse(s: &str) -> Vec<FnDefAst> {
        super::parse(crate::lexer::lex(s.to_owned()).unwrap()).unwrap()
    }

    fn fn_def_ast(left_def: LeftDefAst, expr: ExprAst) -> FnDefAst {
        FnDefAst { left_def, expr }
    }

    fn left_def_ast(ident: IdentAst, args: Vec<IdentAst>) -> LeftDefAst {
        LeftDefAst { ident, args }
    }

    fn fn_expr_ast(fn_ast: FnAst) -> ExprAst {
        ExprAst::Fn(fn_ast)
    }

    fn prefix_op_expr_ast(prefix_op_ast: PrefixOpAst) -> ExprAst {
        ExprAst::PrefixOp(prefix_op_ast)
    }

    fn infix_op_expr_ast(infix_op_ast: InfixOpAst) -> ExprAst {
        ExprAst::InfixOp(infix_op_ast)
    }

    fn ident_expr_ast(ident_ast: IdentAst) -> ExprAst {
        ExprAst::Ident(ident_ast)
    }

    fn num_expr_ast(num_ast: NumAst) -> ExprAst {
        ExprAst::Num(num_ast)
    }

    fn fn_ast(fn_expr: ExprAst, arg_expr: ExprAst) -> FnAst {
        FnAst { fn_expr: Rc::new(fn_expr), arg_expr: Rc::new(arg_expr) }
    }

    fn prefix_op_ast() -> PrefixOpAst {
        panic!("Not implemented.");
    }

    fn infix_op_ast(op_code: &str, lhs: ExprAst, rhs: ExprAst) -> InfixOpAst {
        InfixOpAst { op_code: op_code.to_owned(), lhs: Rc::new(lhs), rhs: Rc::new(rhs) }
    }

    fn ident_ast(name: &str) -> IdentAst {
        IdentAst { name: name.to_owned() }
    }

    fn num_ast(value: &str) -> NumAst {
        NumAst { value: value.to_owned() }
    }

    #[test]
    fn parse_empty() {
        assert_eq!(parse(""), &[]);
    }

    #[test]
    fn parse_arg() {
        assert_eq!(
            parse("f a = 0"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![ident_ast("a")]),
                num_expr_ast(num_ast("0"))
            )]
        );
        assert_eq!(
            parse("f a b = a + b"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![ident_ast("a"), ident_ast("b")]),
                infix_op_expr_ast(infix_op_ast("+", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("b"))))
            )]
        );
    }

    #[test]
    fn parse_ident() {
        assert_eq!(
            parse("f = a"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                ident_expr_ast(ident_ast("a"))
            )]
        );
        assert_eq!(
            parse("f = f"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                ident_expr_ast(ident_ast("f"))
            )]
        );
    }

    #[test]
    fn parse_num() {
        assert_eq!(
            parse("f = 0"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                num_expr_ast(num_ast("0"))
            )]
        );
        assert_eq!(
            parse("f = 123"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                num_expr_ast(num_ast("123"))
            )]
        );
    }

    #[test]
    fn parse_fn() {
        assert_eq!(
            parse("f = g a"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("a"))))
            )]
        );
        assert_eq!(
            parse("f = g a b"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                fn_expr_ast(fn_ast(
                    fn_expr_ast(fn_ast(
                        ident_expr_ast(ident_ast("g")),
                        ident_expr_ast(ident_ast("a"))
                    )),
                    ident_expr_ast(ident_ast("b"))
                ))
            )]
        );
    }

    fn parse_infix_op() {
        assert_eq!(
            parse("f = a + 1"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                infix_op_expr_ast(infix_op_ast("+", ident_expr_ast(ident_ast("a")), num_expr_ast(num_ast("1"))))
            )]
        );
        assert_eq!(
            parse("f = g a + g b + c"),
            &[fn_def_ast(
                left_def_ast(ident_ast("f"), vec![]),
                infix_op_expr_ast(infix_op_ast(
                    "+",
                    infix_op_expr_ast(infix_op_ast(
                        "+",
                        fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("a")))),
                        fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("b"))))
                    )),
                    ident_expr_ast(ident_ast("c"))
                ))
            )]
        );
    }
}
