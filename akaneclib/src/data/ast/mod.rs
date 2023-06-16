mod has_ref_cell;
mod ctor;

pub use has_ref_cell::*;
pub use ctor::*;

use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::data::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TopDefEnum {
    Fn(FnDefAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst {
    pub ty_annot: Option<Rc<TyExprAst>>,
    pub left_fn_def: LeftFnDefAst,
    pub expr: Rc<ExprAst>,
    pub fn_key: RefCell<Option<Rc<FnKey>>>,
    pub arg_sems: RefCell<Option<Rc<Vec<Rc<FnSem>>>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyExprAst {
    pub expr_enum: TyExprEnum,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyExprEnum {
    Arrow(TyArrowAst),
    Ident(TyIdentAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyArrowAst {
    pub lhs: Rc<TyExprAst>,
    pub rhs: Rc<TyExprAst>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyIdentAst {
    pub name: String,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftFnDefAst {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAst {
    pub expr_enum: ExprEnum,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprEnum {
    Fn(FnAst),
    Ident(IdentAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnAst {
    pub fn_expr: Rc<ExprAst>,
    pub arg_expr: Rc<ExprAst>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentAst {
    pub name: String,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
}
