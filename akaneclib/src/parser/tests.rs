use std::rc::Rc;
use crate::data::{
    self,
    *,
};

fn parse<'input>(s: &'input str) -> Vec<TopDefEnum<'input>> {
    super::parse(crate::lexer::lex(s).unwrap()).unwrap()
}

fn top_fn_def_ast<'input>(fn_def_ast: FnDefAst<'input>) -> TopDefEnum<'input> {
    data::top_fn_def_ast(fn_def_ast)
}

fn fn_def_ast<'input>(ty_annot: Option<Rc<TyExprAst<'input>>>, left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>) -> FnDefAst<'input> {
    data::fn_def_ast(ty_annot, left_fn_def, expr, dummy_info())
}

fn ty_arrow_expr_ast<'input>(ty_arrow: TyArrowAst<'input>) -> Rc<TyExprAst<'input>> {
    data::ty_arrow_expr_ast(ty_arrow, dummy_info())
}

fn ty_ident_expr_ast<'input>(ty_ident: TyIdentAst<'input>) -> Rc<TyExprAst<'input>> {
    data::ty_ident_expr_ast(ty_ident, dummy_info())
}

fn ty_arrow_ast<'input>(lhs: Rc<TyExprAst<'input>>, rhs: Rc<TyExprAst<'input>>) -> TyArrowAst<'input> {
    data::ty_arrow_ast(lhs, rhs, dummy_info())
}

fn ty_ident_ast<'input>(name: &'input str) -> TyIdentAst<'input> {
    data::ty_ident_ast(name.to_owned(), dummy_info())
}

fn left_fn_def_ast<'input>(name: &'input str, args: &[&'input str]) -> LeftFnDefAst<'input> {
    data::left_fn_def_ast(name.to_owned(), args.to_owned().into_iter().map(|s| s.to_owned()).collect(), dummy_info())
}

fn fn_expr_ast<'input>(fn_ast: FnAst<'input>) -> Rc<ExprAst<'input>> {
    data::fn_expr_ast(fn_ast, dummy_info())
}

fn ident_expr_ast<'input>(ident_ast: IdentAst<'input>) -> Rc<ExprAst<'input>> {
    data::ident_expr_ast(ident_ast, dummy_info())
}

fn fn_ast<'input>(fn_expr: Rc<ExprAst<'input>>, arg_expr: Rc<ExprAst<'input>>) -> FnAst<'input> {
    data::fn_ast(fn_expr, arg_expr, dummy_info())
}

fn prefix_op_ast<'input>(op_code: &'input str, rhs: Rc<ExprAst<'input>>) -> FnAst<'input> {
    data::prefix_op_ast(op_code.to_owned(), rhs, dummy_info(), dummy_info())
}

fn infix_op_ast<'input>(op_code: &'input str, lhs: Rc<ExprAst<'input>>, rhs: Rc<ExprAst<'input>>) -> FnAst<'input> {
    data::infix_op_ast(op_code.to_owned(), lhs, rhs, dummy_info(), dummy_info(), dummy_info())
}

fn ident_ast<'input>(name: &'input str) -> IdentAst<'input> {
    data::ident_ast(name.to_owned(), dummy_info())
}

fn dummy_info<'a>() -> StrInfo<'a> {
    StrInfo {
        line: 0,
        column: 0,
        slice: "",
        line_slice: "",
    }
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
fn parse_infix_op_prec() {
    assert_eq!(
        parse("fn f = a * b + c"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast(
                    "add",
                    fn_expr_ast(infix_op_ast(
                        "mul",
                        ident_expr_ast(ident_ast("a")),
                        ident_expr_ast(ident_ast("b"))
                    )),
                    ident_expr_ast(ident_ast("c"))
                ))
            )
        )]
    );
    assert_eq!(
        parse("fn f = a + b * c"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast(
                    "add",
                    ident_expr_ast(ident_ast("a")),
                    fn_expr_ast(infix_op_ast(
                        "mul",
                        ident_expr_ast(ident_ast("b")),
                        ident_expr_ast(ident_ast("c"))
                    ))
                ))
            )
        )]
    );
}

#[test]
fn parse_infix_op_right_assoc() {
    assert_eq!(
        parse("fn f = a <| b <| c"),
        &[top_fn_def_ast(
            fn_def_ast(
                None,
                left_fn_def_ast("f", &[]),
                fn_expr_ast(infix_op_ast(
                    "pipelineL",
                    ident_expr_ast(ident_ast("a")),
                    fn_expr_ast(infix_op_ast(
                        "pipelineL",
                        ident_expr_ast(ident_ast("b")),
                        ident_expr_ast(ident_ast("c"))
                    )),
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
