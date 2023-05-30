use std::{
    cell::RefCell,
    rc::Rc,
};
use super::{
    semantics::{
        ty_sem::TySem,
        fn_sem::FnSem,
    },
    thunk::Thunk,
};

#[derive(Clone, Debug, PartialEq)]
pub enum TopDefEnum {
    Fn(FnDefAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst {
    pub ty_annot: Option<TyExprAst>,
    pub left_fn_def: LeftFnDefAst,
    pub expr: ExprAst,
    pub fn_sem: RefCell<Option<Rc<FnSem>>>,
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
    pub thunk: RefCell<Option<Rc<Thunk>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprEnum {
    Fn(FnAst),
    PrefixOp(PrefixOpAst),
    InfixOp(InfixOpAst),
    Ident(IdentAst),
    Num(NumAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnAst {
    pub fn_expr: Rc<ExprAst>,
    pub arg_expr: Rc<ExprAst>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub thunk: RefCell<Option<Rc<Thunk>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixOpAst {
    pub op_code: String,
    pub rhs: Rc<ExprAst>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub thunk: RefCell<Option<Rc<Thunk>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfixOpAst {
    pub op_code: String,
    pub lhs: Rc<ExprAst>,
    pub rhs: Rc<ExprAst>,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub thunk: RefCell<Option<Rc<Thunk>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentAst {
    pub name: String,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub thunk: RefCell<Option<Rc<Thunk>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumAst {
    pub value: String,
    pub ty_sem: RefCell<Option<Rc<TySem>>>,
    pub thunk: RefCell<Option<Rc<Thunk>>>,
}

pub trait HasRefCell<T> {
    fn ref_cell(&self) -> &RefCell<Option<Rc<T>>>;

    fn get_rc(&self) -> Rc<T> {
        self.ref_cell().borrow().as_ref().unwrap().clone()
    }

    fn set_rc(&self, rc: Rc<T>) {
        *self.ref_cell().borrow_mut() = Some(rc);
    }
}

macro_rules! impl_has_ref_cell {
    ($ty:ty, $field:ident, $t:ty) => {
        impl HasRefCell<$t> for $ty {
            fn ref_cell(&self) -> &RefCell<Option<Rc<$t>>> {
                &self.$field
            }
        }
    };
}

impl_has_ref_cell!(FnDefAst, fn_sem, FnSem);
impl_has_ref_cell!(FnDefAst, arg_sems, Vec<Rc<FnSem>>);
impl_has_ref_cell!(TyExprAst, ty_sem, TySem);
impl_has_ref_cell!(TyArrowAst, ty_sem, TySem);
impl_has_ref_cell!(TyIdentAst, ty_sem, TySem);
impl_has_ref_cell!(ExprAst, ty_sem, TySem);
impl_has_ref_cell!(ExprAst, thunk, Thunk);
impl_has_ref_cell!(FnAst, ty_sem, TySem);
impl_has_ref_cell!(FnAst, thunk, Thunk);
impl_has_ref_cell!(PrefixOpAst, ty_sem, TySem);
impl_has_ref_cell!(PrefixOpAst, thunk, Thunk);
impl_has_ref_cell!(InfixOpAst, ty_sem, TySem);
impl_has_ref_cell!(InfixOpAst, thunk, Thunk);
impl_has_ref_cell!(IdentAst, ty_sem, TySem);
impl_has_ref_cell!(IdentAst, thunk, Thunk);
impl_has_ref_cell!(NumAst, ty_sem, TySem);
impl_has_ref_cell!(NumAst, thunk, Thunk);

impl HasRefCell<TySem> for TyExprEnum {
    fn ref_cell(&self) -> &RefCell<Option<Rc<TySem>>> {
        match self {
            Self::Arrow(arrow) => arrow.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
        }
    }
}

impl HasRefCell<TySem> for ExprEnum {
    fn ref_cell(&self) -> &RefCell<Option<Rc<TySem>>> {
        match self {
            Self::Fn(f) => f.ref_cell(),
            Self::PrefixOp(prefix_op) => prefix_op.ref_cell(),
            Self::InfixOp(infix_op) => infix_op.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
            Self::Num(num) => num.ref_cell(),
        }
    }
}

impl HasRefCell<Thunk> for ExprEnum {
    fn ref_cell(&self) -> &RefCell<Option<Rc<Thunk>>> {
        match self {
            Self::Fn(f) => f.ref_cell(),
            Self::PrefixOp(prefix_op) => prefix_op.ref_cell(),
            Self::InfixOp(infix_op) => infix_op.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
            Self::Num(num) => num.ref_cell(),
        }
    }
}
