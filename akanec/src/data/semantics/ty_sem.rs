use std::{
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};
use anyhow::Result;
use crate::{
    impl_sem_key,
    data::context::SemContext,
};
use super::{
    Sem,
    SemVal,
    qual_sem::{
        QualKey,
        QualSem,
    },
    ty2_sem::Ty2Sem,
    ty1_sem::Ty1Sem,
};

#[derive(Clone, Debug)]
pub enum TySem {
    Ty2(Rc<Ty2Sem>),
    Ty1(Rc<Ty1Sem>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TyKey {
    pub qual: QualKey,
    pub name: String,
}

impl_sem_key!(TyKey, TySem, ty_store);

impl PartialEq for TySem {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Ty2(ty2) =>
                match other {
                    Self::Ty2(other_ty2) => ty2.id == other_ty2.id,
                    Self::Ty1(_) => false,
                },
            Self::Ty1(ty1) =>
                match other {
                    Self::Ty2(_) => false,
                    Self::Ty1(other_ty1) => ty1.id == other_ty1.id,
                },
        }
    }
}

impl Eq for TySem {}

impl Hash for TySem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Ty2(ty2) => (ty2.id as i128).hash(state),
            Self::Ty1(ty1) => (-(ty1.id as i128)).hash(state),
        };
    }
}

impl Sem for TySem {
    fn logical_name(&self) -> String {
        self.to_key().logical_name()
    }

    fn description(&self) -> String {
        self.to_key().description()
    }
}

impl SemVal<TyKey> for TySem {
    fn to_key(&self) -> TyKey {
        match self {
            Self::Ty2(ty2) => TyKey { qual: ty2.qual.to_key(), name: ty2.name.clone() },
            Self::Ty1(ty1) => TyKey { qual: ty1.qual.to_key(), name: ty1.name.clone() },
        }
    }
}

impl Sem for TyKey {
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

impl TySem {
    pub fn new_or_get_ty1(ctx: &mut SemContext, qual: Rc<QualSem>, name: String) -> Rc<Self> {
        let val = Rc::new(Self::Ty1(Ty1Sem::new_or_get(ctx, qual, name)));
        let key = val.to_key();
        ctx.ty_store.insert_or_get(key, val)
    }

    pub fn new_or_get_ty2(ctx: &mut SemContext, qual: Rc<QualSem>, in_ty: Rc<TySem>, out_ty: Rc<TySem>) -> Rc<Self> {
        let val = Rc::new(Self::Ty2(Ty2Sem::new_or_get(ctx, qual, in_ty, out_ty)));
        let key = val.to_key();
        ctx.ty_store.insert_or_get(key, val)
    }

    pub fn new_or_get_fn_ty(ctx: &mut SemContext, qual: Rc<QualSem>, in_tys: Vec<Rc<TySem>>, out_ty: Rc<TySem>) -> Result<Rc<Self>> {
        let mut ty = out_ty;
        for in_ty in in_tys.into_iter().rev() {
            ty = Self::new_or_get_ty2(ctx, qual.clone(), in_ty, ty);
        }
        Ok(ty)
    }

    pub fn get_from_name(ctx: &SemContext, name: &str) -> Result<Rc<Self>> {
        ctx.ty_store.get(&TyKey::from_name(name))
    }

    pub fn rank(&self) -> usize {
        match self {
            TySem::Ty2(ty2) => ty2.rank,
            TySem::Ty1(_) => 0,
        }
    }
}

impl TyKey {
    pub fn new(qual: QualKey, name: String) -> Self {
        Self { qual, name }
    }

    pub fn from_name(name: &str) -> Self {
        Self {
            qual: QualKey::top(),
            name: name.to_owned(),
        }
    }
}
