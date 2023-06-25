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
pub enum TopDefEnum<'input> {
    Fn(FnDefAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst<'input> {
    pub ty_annot: Option<Rc<TyExprAst<'input>>>,
    pub left_fn_def: LeftFnDefAst<'input>,
    pub expr: Rc<ExprAst<'input>>,
    pub fn_key: RefCell<Option<Rc<FnKey>>>,
    pub arg_sems: RefCell<Option<Rc<Vec<Rc<FnSem>>>>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyExprAst<'input> {
    pub expr_enum: TyExprEnum<'input>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyExprEnum<'input> {
    Arrow(TyArrowAst<'input>),
    Ident(TyIdentAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyArrowAst<'input> {
    pub lhs: Rc<TyExprAst<'input>>,
    pub rhs: Rc<TyExprAst<'input>>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TyIdentAst<'input> {
    pub name: String,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftFnDefAst<'input> {
    pub name: String,
    pub args: Vec<String>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAst<'input> {
    pub expr_enum: ExprEnum<'input>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprEnum<'input> {
    Fn(FnAst<'input>),
    Ident(IdentAst<'input>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnAst<'input> {
    pub fn_expr: Rc<ExprAst<'input>>,
    pub arg_expr: Rc<ExprAst<'input>>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
    pub str_info: StrInfo<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentAst<'input> {
    pub name: String,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
    pub str_info: StrInfo<'input>,
}
