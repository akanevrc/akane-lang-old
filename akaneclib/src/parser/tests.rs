use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::data::ast::{
    TopDefEnum,
    FnDefAst,
    TyExprAst,
    TyExprEnum,
    TyArrowAst,
    TyIdentAst,
    LeftFnDefAst,
    ExprAst,
    ExprEnum,
    FnAst,
    PrefixOpAst,
    InfixOpAst,
    IdentAst,
    NumAst,
};

fn parse(s: &str) -> Vec<TopDefEnum> {
    super::parse(crate::lexer::lex(s.to_owned()).unwrap()).unwrap()
}

fn top_fn_def_ast(fn_def_ast: FnDefAst) -> TopDefEnum {
    TopDefEnum::Fn(fn_def_ast)
}

fn fn_def_ast(ty_annot: Option<TyExprAst>, left_fn_def: LeftFnDefAst, expr: ExprAst) -> FnDefAst {
    FnDefAst { ty_annot, left_fn_def, expr, fn_sem: RefCell::new(None), arg_sems: RefCell::new(None) }
}

fn ty_arrow_expr_ast(ty_arrow: TyArrowAst) -> TyExprAst {
    TyExprAst { expr_enum: TyExprEnum::Arrow(ty_arrow), ty_sem: RefCell::new(None) }
}

fn ty_ident_expr_ast(ty_ident: TyIdentAst) -> TyExprAst {
    TyExprAst { expr_enum: TyExprEnum::Ident(ty_ident), ty_sem: RefCell::new(None) }
}

fn ty_arrow_ast(lhs: TyExprAst, rhs: TyExprAst) -> TyArrowAst {
    TyArrowAst { lhs: Rc::new(lhs), rhs: Rc::new(rhs) }
}

fn ty_ident_ast(name: &str) -> TyIdentAst {
    TyIdentAst { name: name.to_owned() }
}

fn left_fn_def_ast(name: &str, args: Vec<&str>) -> LeftFnDefAst {
    LeftFnDefAst { name: name.to_owned(), args: args.into_iter().map(|s| s.to_owned()).collect() }
}

fn fn_expr_ast(fn_ast: FnAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::Fn(fn_ast), ty_sem: RefCell::new(None) }
}

fn prefix_op_expr_ast(prefix_op_ast: PrefixOpAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::PrefixOp(prefix_op_ast), ty_sem: RefCell::new(None) }
}

fn infix_op_expr_ast(infix_op_ast: InfixOpAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::InfixOp(infix_op_ast), ty_sem: RefCell::new(None) }
}

fn ident_expr_ast(ident_ast: IdentAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::Ident(ident_ast), ty_sem: RefCell::new(None) }
}

fn num_expr_ast(num_ast: NumAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::Num(num_ast), ty_sem: RefCell::new(None) }
}

fn fn_ast(fn_expr: ExprAst, arg_expr: ExprAst) -> FnAst {
    FnAst { fn_expr: Rc::new(fn_expr), arg_expr: Rc::new(arg_expr), thunk: RefCell::new(None) }
}

fn prefix_op_ast(op_code: &str, rhs: ExprAst) -> PrefixOpAst {
    PrefixOpAst { op_code: op_code.to_owned(), rhs: Rc::new(rhs) }
}

fn infix_op_ast(op_code: &str, lhs: ExprAst, rhs: ExprAst) -> InfixOpAst {
    InfixOpAst { op_code: op_code.to_owned(), lhs: Rc::new(lhs), rhs: Rc::new(rhs) }
}

fn ident_ast(name: &str) -> IdentAst {
    IdentAst { name: name.to_owned(), thunk: RefCell::new(None) }
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
        parse("fn f a = 0"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec!["a"]),
                num_expr_ast(num_ast("0"))
            )
        )]
    );
    assert_eq!(
        parse("fn f a b = a + b"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec!["a", "b"]),
                infix_op_expr_ast(infix_op_ast("+", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("b"))))
            )
        )]
    );
}

#[test]
fn parse_ident() {
    assert_eq!(
        parse("fn f = a"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                ident_expr_ast(ident_ast("a"))
            )
        )]
    );
    assert_eq!(
        parse("fn f = f"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                ident_expr_ast(ident_ast("f"))
            )
        )]
    );
}

#[test]
fn parse_num() {
    assert_eq!(
        parse("fn f = 0"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                num_expr_ast(num_ast("0"))
            )
        )]
    );
    assert_eq!(
        parse("fn f = 123"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                num_expr_ast(num_ast("123"))
            )
        )]
    );
}

#[test]
fn parse_fn() {
    assert_eq!(
        parse("fn f = g a"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("a"))))
            )
        )]
    );
    assert_eq!(
        parse("fn f = g a b"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                fn_expr_ast(fn_ast(
                    fn_expr_ast(fn_ast(
                        ident_expr_ast(ident_ast("g")),
                        ident_expr_ast(ident_ast("a"))
                    )),
                    ident_expr_ast(ident_ast("b"))
                ))
            )
        )]
    );
}

#[test]
fn parse_infix_op() {
    assert_eq!(
        parse("fn f = a + 1"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                infix_op_expr_ast(infix_op_ast("+", ident_expr_ast(ident_ast("a")), num_expr_ast(num_ast("1"))))
            )
        )]
    );
    assert_eq!(
        parse("fn f = g a + g b + c"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                infix_op_expr_ast(infix_op_ast(
                    "+",
                    infix_op_expr_ast(infix_op_ast(
                        "+",
                        fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("a")))),
                        fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("b"))))
                    )),
                    ident_expr_ast(ident_ast("c"))
                ))
            )
        )]
    );
}

#[test]
fn parse_paren() {
    assert_eq!(
        parse("fn f = (a + b) + c"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                infix_op_expr_ast(infix_op_ast(
                    "+",
                    infix_op_expr_ast(infix_op_ast(
                        "+",
                        ident_expr_ast(ident_ast("a")),
                        ident_expr_ast(ident_ast("b"))
                    )),
                    ident_expr_ast(ident_ast("c"))
                ))
            )
        )]
    );
    assert_eq!(
        parse("fn f = a + (b + c)"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                infix_op_expr_ast(infix_op_ast(
                    "+",
                    ident_expr_ast(ident_ast("a")),
                    infix_op_expr_ast(infix_op_ast(
                        "+",
                        ident_expr_ast(ident_ast("b")),
                        ident_expr_ast(ident_ast("c"))
                    ))
                ))
            )
        )]
    );
}

#[test]
fn parse_prefix_op() {
    assert_eq!(
        parse("fn f = -1"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                prefix_op_expr_ast(prefix_op_ast("-", num_expr_ast(num_ast("1"))))
            )
        )]
    );
    assert_eq!(
        parse("fn f = -a + 1"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", vec![]),
                infix_op_expr_ast(
                    infix_op_ast(
                        "+",
                        prefix_op_expr_ast(prefix_op_ast("-", ident_expr_ast(ident_ast("a")))),
                        num_expr_ast(num_ast("1"))
                    )
                )
            )
        )]
    );
}

#[test]
fn parse_ty_annot() {
    assert_eq!(
        parse("ty i32 fn f = 0"),
        &[top_fn_def_ast(
            fn_def_ast(
                Some(ty_ident_expr_ast(ty_ident_ast("i32"))),
                left_fn_def_ast("f", vec![]),
                num_expr_ast(num_ast("0"))
            )
        )]
    );
    assert_eq!(
        parse("ty i32 -> i32 -> i32 fn f a b = a + b"),
        &[top_fn_def_ast(
            fn_def_ast(
                Some(ty_arrow_expr_ast(ty_arrow_ast(
                    ty_ident_expr_ast(ty_ident_ast("i32")),
                    ty_arrow_expr_ast(ty_arrow_ast(
                        ty_ident_expr_ast(ty_ident_ast("i32")),
                        ty_ident_expr_ast(ty_ident_ast("i32")),
                    ))
                ))),
                left_fn_def_ast("f", vec!["a", "b"]),
                infix_op_expr_ast(infix_op_ast("+", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("b"))))
            )
        )]
    );
}
