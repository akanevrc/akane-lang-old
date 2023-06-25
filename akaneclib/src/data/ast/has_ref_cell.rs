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
        impl<'input> HasRefCell<$t> for $ty {
            fn ref_cell(&self) -> &RefCell<Option<Rc<$t>>> {
                &self.$field
            }
        }
    };
}

impl_has_ref_cell!(FnDefAst<'input>, fn_key, FnKey);
impl_has_ref_cell!(FnDefAst<'input>, arg_sems, Vec<Rc<FnSem>>);
impl_has_ref_cell!(TyExprAst<'input>, ty_sem, TySem);
impl_has_ref_cell!(TyArrowAst<'input>, ty_sem, TySem);
impl_has_ref_cell!(TyIdentAst<'input>, ty_sem, TySem);
impl_has_ref_cell!(ExprAst<'input>, ty_sem, TySem);
impl_has_ref_cell!(ExprAst<'input>, fn_sem, FnSem);
impl_has_ref_cell!(FnAst<'input>, ty_sem, TySem);
impl_has_ref_cell!(FnAst<'input>, fn_sem, FnSem);
impl_has_ref_cell!(IdentAst<'input>, ty_sem, TySem);
impl_has_ref_cell!(IdentAst<'input>, fn_sem, FnSem);

impl<'input> HasRefCell<TySem> for TyExprEnum<'input> {
    fn ref_cell(&self) -> &RefCell<Option<Rc<TySem>>> {
        match self {
            Self::Arrow(arrow) => arrow.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
        }
    }
}

impl<'input> HasRefCell<TySem> for ExprEnum<'input> {
    fn ref_cell(&self) -> &RefCell<Option<Rc<TySem>>> {
        match self {
            Self::Fn(f) => f.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
        }
    }
}

impl<'input> HasRefCell<FnSem> for ExprEnum<'input> {
    fn ref_cell(&self) -> &RefCell<Option<Rc<FnSem>>> {
        match self {
            Self::Fn(f) => f.ref_cell(),
            Self::Ident(ident) => ident.ref_cell(),
        }
    }
}
