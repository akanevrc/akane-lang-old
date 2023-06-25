use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::data::*;

pub fn top_fn_def_ast<'input>(fn_def_ast: FnDefAst<'input>) -> TopDefEnum<'input> {
    TopDefEnum::Fn(fn_def_ast)
}

pub fn fn_def_ast<'input>(ty_annot: Option<Rc<TyExprAst<'input>>>, left_fn_def: LeftFnDefAst<'input>, expr: Rc<ExprAst<'input>>, str_info: StrInfo<'input>) -> FnDefAst<'input> {
    FnDefAst { ty_annot, left_fn_def, expr, fn_key: RefCell::new(None), arg_sems: RefCell::new(None), str_info }
}

pub fn ty_arrow_expr_ast<'input>(ty_arrow: TyArrowAst<'input>, str_info: StrInfo<'input>) -> Rc<TyExprAst<'input>> {
    Rc::new(TyExprAst { expr_enum: TyExprEnum::Arrow(ty_arrow), ty_sem: RefCell::new(None), str_info })
}

pub fn ty_ident_expr_ast<'input>(ty_ident: TyIdentAst<'input>, str_info: StrInfo<'input>) -> Rc<TyExprAst<'input>> {
    Rc::new(TyExprAst { expr_enum: TyExprEnum::Ident(ty_ident), ty_sem: RefCell::new(None), str_info })
}

pub fn ty_arrow_ast<'input>(lhs: Rc<TyExprAst<'input>>, rhs: Rc<TyExprAst<'input>>, str_info: StrInfo<'input>) -> TyArrowAst<'input> {
    TyArrowAst { lhs, rhs, ty_sem: RefCell::new(None), str_info }
}

pub fn ty_ident_ast<'input>(name: String, str_info: StrInfo<'input>) -> TyIdentAst<'input> {
    TyIdentAst { name, ty_sem: RefCell::new(None), str_info }
}

pub fn left_fn_def_ast<'input>(name: String, args: Vec<String>, str_info: StrInfo<'input>) -> LeftFnDefAst<'input> {
    LeftFnDefAst { name, args, str_info }
}

pub fn fn_expr_ast<'input>(fn_ast: FnAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Fn(fn_ast), ty_sem: RefCell::new(None), fn_sem: RefCell::new(None), str_info })
}

pub fn ident_expr_ast<'input>(ident_ast: IdentAst<'input>, str_info: StrInfo<'input>) -> Rc<ExprAst<'input>> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Ident(ident_ast), ty_sem: RefCell::new(None), fn_sem: RefCell::new(None), str_info })
}

pub fn fn_ast<'input>(fn_expr: Rc<ExprAst<'input>>, arg_expr: Rc<ExprAst<'input>>, str_info: StrInfo<'input>) -> FnAst<'input> {
    FnAst { fn_expr, arg_expr, ty_sem: RefCell::new(None), fn_sem: RefCell::new(None), str_info }
}

pub fn prefix_op_ast<'input>(op_code: String, rhs: Rc<ExprAst<'input>>, str_info: StrInfo<'input>, op_code_info: StrInfo<'input>) -> FnAst<'input> {
    fn_ast(
        ident_expr_ast(ident_ast(op_code.to_owned(), op_code_info.clone()), op_code_info),
        rhs,
        str_info,
    )
}

pub fn infix_op_ast<'input>(op_code: String, lhs: Rc<ExprAst<'input>>, rhs: Rc<ExprAst<'input>>, str_info: StrInfo<'input>, op_code_info: StrInfo<'input>, lhs_info: StrInfo<'input>) -> FnAst<'input> {
    fn_ast(
        fn_expr_ast(
            fn_ast(
                ident_expr_ast(ident_ast(op_code, op_code_info.clone()), op_code_info),
                lhs,
                lhs_info.clone(),
            ),
            lhs_info,
        ),
        rhs,
        str_info,
    )
}

pub fn ident_ast<'input>(name: String, str_info: StrInfo<'input>) -> IdentAst<'input> {
    IdentAst { name, ty_sem: RefCell::new(None), fn_sem: RefCell::new(None), str_info }
}
