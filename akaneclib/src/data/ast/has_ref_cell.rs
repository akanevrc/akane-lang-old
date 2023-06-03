use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::data::*;

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
impl_has_ref_cell!(IdentAst, ty_sem, TySem);
impl_has_ref_cell!(IdentAst, thunk, Thunk);

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
            Self::Ident(ident) => ident.ref_cell(),
        }
    }
}

impl HasRefCell<Thunk> for ExprEnum {
    fn ref_cell(&self) -> &RefCell<Option<Rc<Thunk>>> {
        match self {
            Self::Fn(f) => f.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
        }
    }
}
