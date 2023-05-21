use std::rc::Rc;
use crate::data::ast::{
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

fn prefix_op_ast(op_code: &str, rhs: ExprAst) -> PrefixOpAst {
    PrefixOpAst { op_code: op_code.to_owned(), rhs: Rc::new(rhs) }
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

#[test]
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

#[test]
fn parse_paren() {
    assert_eq!(
        parse("f = (a + b) + c"),
        &[fn_def_ast(
            left_def_ast(ident_ast("f"), vec![]),
            infix_op_expr_ast(infix_op_ast(
                "+",
                infix_op_expr_ast(infix_op_ast(
                    "+",
                    ident_expr_ast(ident_ast("a")),
                    ident_expr_ast(ident_ast("b"))
                )),
                ident_expr_ast(ident_ast("c"))
            ))
        )]
    );
    assert_eq!(
        parse("f = a + (b + c)"),
        &[fn_def_ast(
            left_def_ast(ident_ast("f"), vec![]),
            infix_op_expr_ast(infix_op_ast(
                "+",
                ident_expr_ast(ident_ast("a")),
                infix_op_expr_ast(infix_op_ast(
                    "+",
                    ident_expr_ast(ident_ast("b")),
                    ident_expr_ast(ident_ast("c"))
                ))
            ))
        )]
    );
}

#[test]
fn parse_prefix_op() {
    assert_eq!(
        parse("f = -1"),
        &[fn_def_ast(
            left_def_ast(ident_ast("f"), vec![]),
            prefix_op_expr_ast(prefix_op_ast("-", num_expr_ast(num_ast("1"))))
        )]
    );
    assert_eq!(
        parse("f = -a + 1"),
        &[fn_def_ast(
            left_def_ast(ident_ast("f"), vec![]),
            infix_op_expr_ast(
                infix_op_ast(
                    "+",
                    prefix_op_expr_ast(prefix_op_ast("-", ident_expr_ast(ident_ast("a")))),
                    num_expr_ast(num_ast("1"))
                )
            )
        )]
    );
}
