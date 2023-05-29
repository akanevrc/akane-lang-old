pub mod qual_stack;
pub mod store;

use std::rc::Rc;
use super::semantics::{
    SemKey,
    scope_sem::ScopeSem,
    qual_sem::{
        QualSem,
        QualKey,
    },
    ty_sem::{
        TySem,
        TyKey,
    },
    ty2_sem::{
        Ty2Sem,
        Ty2Key,
    },
    ty1_sem::{
        Ty1Sem,
        Ty1Key,
    },
    fn_sem::{
        FnSem,
        FnKey,
    },
};
use self::{
    qual_stack::QualStack,
    store::Store,
};

pub struct SemContext {
    pub qual_stack: QualStack,
    pub qual_store: Store<QualKey, QualSem>,
    pub ty_store: Store<TyKey, TySem>,
    pub ty2_store: Store<Ty2Key, Ty2Sem>,
    pub ty1_store: Store<Ty1Key, Ty1Sem>,
    pub fn_store: Store<FnKey, FnSem>,
}

impl SemContext {
    pub fn new() -> Self {
        let mut ctx = Self {
            qual_stack: QualStack::new(),
            qual_store: Store::<QualKey, QualSem>::new(),
            ty_store: Store::<TyKey, TySem>::new(),
            ty2_store: Store::<Ty2Key, Ty2Sem>::new(),
            ty1_store: Store::<Ty1Key, Ty1Sem>::new(),
            fn_store: Store::<FnKey, FnSem>::new(),
        };
        let top = QualSem::top(&mut ctx);
        TySem::new_or_get_ty1(&mut ctx, top, "i32".to_owned());
        ctx
    }

    pub fn push_scope_into_qual_stack(&mut self, scope: ScopeSem) -> QualKey {
        let qual_key = self.qual_stack.peek().pushed(scope);
        let qual = QualSem::new_or_get(self, &qual_key);
        self.qual_stack.push(&qual)
    }

    pub fn find_with_qual<T>(&self, f: impl Fn(&Self, Rc<QualSem>) -> Option<T>) -> Option<T> {
        for qual in self.qual_stack.iter() {
            match f(self, qual.get_val(self).unwrap()) {
                Some(x) => return Some(x),
                None => (),
            }
        }
        None
    }
}
