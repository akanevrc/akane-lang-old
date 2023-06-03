mod qual_stack;
mod store;

pub use qual_stack::*;
pub use store::*;

use std::rc::Rc;
use crate::data::*;

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
        let i32_ty = TySem::new_or_get_ty1(&mut ctx, top.clone(), "i32".to_owned());
        let i32_bin_op_ty = TySem::new_or_get_fn_ty(&mut ctx, top.clone(), vec![i32_ty.clone(), i32_ty.clone()], i32_ty);
        FnSem::new_or_get(&mut ctx, top, "+".to_owned(), i32_bin_op_ty);
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
