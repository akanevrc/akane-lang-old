use std::{
    cell::RefCell,
    rc::Rc,
};
use super::{
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
    IdentAst,
};

pub fn top_fn_def_ast(fn_def_ast: FnDefAst) -> TopDefEnum {
    TopDefEnum::Fn(fn_def_ast)
}

pub fn fn_def_ast(ty_annot: Option<Rc<TyExprAst>>, left_fn_def: LeftFnDefAst, expr: Rc<ExprAst>) -> FnDefAst {
    FnDefAst { ty_annot, left_fn_def, expr, fn_key: RefCell::new(None), arg_sems: RefCell::new(None) }
}

pub fn ty_arrow_expr_ast(ty_arrow: TyArrowAst) -> Rc<TyExprAst> {
    Rc::new(TyExprAst { expr_enum: TyExprEnum::Arrow(ty_arrow), ty_sem: RefCell::new(None) })
}

pub fn ty_ident_expr_ast(ty_ident: TyIdentAst) -> Rc<TyExprAst> {
    Rc::new(TyExprAst { expr_enum: TyExprEnum::Ident(ty_ident), ty_sem: RefCell::new(None) })
}

pub fn ty_arrow_ast(lhs: Rc<TyExprAst>, rhs: Rc<TyExprAst>) -> TyArrowAst {
    TyArrowAst { lhs, rhs, ty_sem: RefCell::new(None) }
}

pub fn ty_ident_ast(name: String) -> TyIdentAst {
    TyIdentAst { name, ty_sem: RefCell::new(None) }
}

pub fn left_fn_def_ast(name: String, args: Vec<String>) -> LeftFnDefAst {
    LeftFnDefAst { name, args }
}

pub fn fn_expr_ast(fn_ast: FnAst) -> Rc<ExprAst> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Fn(fn_ast), ty_sem: RefCell::new(None), fn_sem: RefCell::new(None) })
}

pub fn ident_expr_ast(ident_ast: IdentAst) -> Rc<ExprAst> {
    Rc::new(ExprAst { expr_enum: ExprEnum::Ident(ident_ast), ty_sem: RefCell::new(None), fn_sem: RefCell::new(None) })
}

pub fn fn_ast(fn_expr: Rc<ExprAst>, arg_expr: Rc<ExprAst>) -> FnAst {
    FnAst { fn_expr, arg_expr, ty_sem: RefCell::new(None), fn_sem: RefCell::new(None) }
}

pub fn prefix_op_ast(op_code: String, rhs: Rc<ExprAst>) -> FnAst {
    fn_ast(
        ident_expr_ast(ident_ast(op_code.to_owned())),
        rhs,
    )
}

pub fn infix_op_ast(op_code: String, lhs: Rc<ExprAst>, rhs: Rc<ExprAst>) -> FnAst {
    fn_ast(
        fn_expr_ast(fn_ast(
            ident_expr_ast(ident_ast(op_code)),
            lhs,
        )),
        rhs,
    )
}

pub fn ident_ast(name: String) -> IdentAst {
    IdentAst { name, ty_sem: RefCell::new(None), fn_sem: RefCell::new(None) }
}
