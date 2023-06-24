use std::rc::Rc;
use crate::data::*;

fn parse(s: &str) -> Vec<TopDefEnum> {
    super::parse(crate::lexer::lex(s).unwrap()).unwrap()
}

fn ty_ident_ast(name: &str) -> TyIdentAst {
    crate::data::ty_ident_ast(name.to_owned())
}

fn left_fn_def_ast(name: &str, args: &[&str]) -> LeftFnDefAst {
    crate::data::left_fn_def_ast(name.to_owned(), args.to_owned().into_iter().map(|s| s.to_owned()).collect())
}

fn prefix_op_ast(op_code: &str, rhs: Rc<ExprAst>) -> FnAst {
    crate::data::prefix_op_ast(op_code.to_owned(), rhs)
}

fn infix_op_ast(op_code: &str, lhs: Rc<ExprAst>, rhs: Rc<ExprAst>) -> FnAst {
    crate::data::infix_op_ast(op_code.to_owned(), lhs, rhs)
}

fn ident_ast(name: &str) -> IdentAst {
    crate::data::ident_ast(name.to_owned())
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
                left_fn_def_ast("f", &["a"]),
                ident_expr_ast(ident_ast("0"))
            )
        )]
    );
    assert_eq!(
        parse("fn f a b = a + b"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &["a", "b"]),
                fn_expr_ast(infix_op_ast("add", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("b"))))
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
                left_fn_def_ast("f", &[]),
                ident_expr_ast(ident_ast("a"))
            )
        )]
    );
    assert_eq!(
        parse("fn f = f"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
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
                left_fn_def_ast("f", &[]),
                ident_expr_ast(ident_ast("0"))
            )
        )]
    );
    assert_eq!(
        parse("fn f = 123"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
                ident_expr_ast(ident_ast("123"))
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
                left_fn_def_ast("f", &[]),
                fn_expr_ast(fn_ast(ident_expr_ast(ident_ast("g")), ident_expr_ast(ident_ast("a"))))
            )
        )]
    );
    assert_eq!(
        parse("fn f = g a b"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
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
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast("add", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("1"))))
            )
        )]
    );
    assert_eq!(
        parse("fn f = g a + g b + c"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast(
                    "add",
                    fn_expr_ast(infix_op_ast(
                        "add",
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
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast(
                    "add",
                    fn_expr_ast(infix_op_ast(
                        "add",
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
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast(
                    "add",
                    ident_expr_ast(ident_ast("a")),
                    fn_expr_ast(infix_op_ast(
                        "add",
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
                left_fn_def_ast("f", &[]),
                fn_expr_ast(prefix_op_ast("negate", ident_expr_ast(ident_ast("1"))))
            )
        )]
    );
    assert_eq!(
        parse("fn f = -a + 1"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
                fn_expr_ast(
                    infix_op_ast(
                        "add",
                        fn_expr_ast(prefix_op_ast("negate", ident_expr_ast(ident_ast("a")))),
                        ident_expr_ast(ident_ast("1"))
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
                left_fn_def_ast("f", &[]),
                ident_expr_ast(ident_ast("0"))
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
                left_fn_def_ast("f", &["a", "b"]),
                fn_expr_ast(infix_op_ast("add", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("b"))))
            )
        )]
    );
    assert_eq!(
        parse("ty (i32 -> i32) -> i32 fn f a b = a + b"),
        &[top_fn_def_ast(
            fn_def_ast(
                Some(ty_arrow_expr_ast(ty_arrow_ast(
                    ty_arrow_expr_ast(ty_arrow_ast(
                        ty_ident_expr_ast(ty_ident_ast("i32")),
                        ty_ident_expr_ast(ty_ident_ast("i32")),
                    )),
                    ty_ident_expr_ast(ty_ident_ast("i32")),
                ))),
                left_fn_def_ast("f", &["a", "b"]),
                fn_expr_ast(infix_op_ast("add", ident_expr_ast(ident_ast("a")), ident_expr_ast(ident_ast("b"))))
            )
        )]
    );
}
