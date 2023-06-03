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
    PrefixOpAst,
    InfixOpAst,
    IdentAst,
    NumAst,
};

pub fn top_fn_def_ast(fn_def_ast: FnDefAst) -> TopDefEnum {
    TopDefEnum::Fn(fn_def_ast)
}

pub fn fn_def_ast(ty_annot: Option<TyExprAst>, left_fn_def: LeftFnDefAst, expr: ExprAst) -> FnDefAst {
    FnDefAst { ty_annot, left_fn_def, expr, fn_sem: RefCell::new(None), arg_sems: RefCell::new(None) }
}

pub fn ty_arrow_expr_ast(ty_arrow: TyArrowAst) -> TyExprAst {
    TyExprAst { expr_enum: TyExprEnum::Arrow(ty_arrow), ty_sem: RefCell::new(None) }
}

pub fn ty_ident_expr_ast(ty_ident: TyIdentAst) -> TyExprAst {
    TyExprAst { expr_enum: TyExprEnum::Ident(ty_ident), ty_sem: RefCell::new(None) }
}

pub fn ty_arrow_ast(lhs: TyExprAst, rhs: TyExprAst) -> TyArrowAst {
    TyArrowAst { lhs: Rc::new(lhs), rhs: Rc::new(rhs), ty_sem: RefCell::new(None) }
}

pub fn ty_ident_ast(name: String) -> TyIdentAst {
    TyIdentAst { name, ty_sem: RefCell::new(None) }
}

pub fn left_fn_def_ast(name: String, args: Vec<String>) -> LeftFnDefAst {
    LeftFnDefAst { name, args }
}

pub fn fn_expr_ast(fn_ast: FnAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::Fn(fn_ast), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn prefix_op_expr_ast(prefix_op_ast: PrefixOpAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::PrefixOp(prefix_op_ast), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn infix_op_expr_ast(infix_op_ast: InfixOpAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::InfixOp(infix_op_ast), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn ident_expr_ast(ident_ast: IdentAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::Ident(ident_ast), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn num_expr_ast(num_ast: NumAst) -> ExprAst {
    ExprAst { expr_enum: ExprEnum::Num(num_ast), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn fn_ast(fn_expr: ExprAst, arg_expr: ExprAst) -> FnAst {
    FnAst { fn_expr: Rc::new(fn_expr), arg_expr: Rc::new(arg_expr), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn prefix_op_ast(op_code: String, rhs: ExprAst) -> PrefixOpAst {
    PrefixOpAst { op_code, rhs: Rc::new(rhs), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn infix_op_ast(op_code: String, lhs: ExprAst, rhs: ExprAst) -> InfixOpAst {
    InfixOpAst { op_code, lhs: Rc::new(lhs), rhs: Rc::new(rhs), ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn ident_ast(name: String) -> IdentAst {
    IdentAst { name, ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}

pub fn num_ast(value: String) -> NumAst {
    NumAst { value, ty_sem: RefCell::new(None), thunk: RefCell::new(None) }
}
