use std::rc::Rc;
use anyhow::Result;
use crate::{
    impl_sem_val,
    impl_sem_key,
    data::context::SemContext,
};
use super::{
    Sem,
    SemVal,
    qual_sem::{
        QualSem,
        QualKey,
    },
    ty_sem::TySem,
};

#[derive(Clone, Debug)]
pub struct FnSem {
    pub id: usize,
    pub qual: Rc<QualSem>,
    pub name: String,
    pub ty: Rc<TySem>,
    pub rank: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FnKey {
    pub qual: QualKey,
    pub name: String,
}

impl_sem_val!(
    FnKey,
    FnSem,
    FnKey {
        qual: self.qual.to_key(),
        name: self.name.clone()
    }
);
impl_sem_key!(FnKey, FnSem, fn_store);

impl Sem for FnKey {
    fn logical_name(&self) -> String {
        format!(
            "{}{}",
            self.qual.qualify_logical_name("."),
            self.name.logical_name()
        )
    }

    fn description(&self) -> String {
        format!(
            "{}{}",
            self.qual.qualify_description("."),
            self.name.description()
        )
    }
}

impl FnSem {
    pub fn new(ctx: &mut SemContext, qual: Rc<QualSem>, name: String, ty: Rc<TySem>) -> Result<Rc<Self>> {
        let rank = ty.rank();
        let val = Rc::new(Self {
            id: ctx.fn_store.next_id(),
            qual,
            name,
            ty,
            rank,
        });
        let key = val.to_key();
        ctx.fn_store.insert(key, val)
    }

    pub fn new_or_get(ctx: &mut SemContext, qual: Rc<QualSem>, name: String, ty: Rc<TySem>) -> Rc<Self> {
        let rank = ty.rank();
        let val = Rc::new(Self {
            id: ctx.fn_store.next_id(),
            qual,
            name,
            ty,
            rank,
        });
        let key = val.to_key();
        ctx.fn_store.insert_or_get(key, val)
    }

    pub fn get(ctx: &SemContext, qual: QualKey, name: String) -> Result<Rc<Self>> {
        let key = FnKey::new(qual, name);
        key.get(ctx)
    }
}

impl FnKey {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }

    pub fn get(&self, ctx: &SemContext) -> Result<Rc<FnSem>> {
        ctx.fn_store.get(self)
    }
}
